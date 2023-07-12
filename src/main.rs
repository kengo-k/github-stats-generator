mod query;

#[macro_use]
extern crate rocket;

use crate::query::github_stats;
use graphql_client::GraphQLQuery;
use query::github_stats::ResponseData;
use query::GithubStats;
use reqwest::Client;
use rocket::http::{ContentType, Status};
use rocket::info;
use rocket::response::Responder;
use rocket::{response, Request, Response};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::string::ToString;
use svg::node::element::{Definitions, LinearGradient, Rectangle, Stop, Text};
use svg::Document;

#[derive(Debug)]
enum AppError {
    GetJsonSourceError, // SVG生成の元になる入力データの取得に失敗したことを示すエラー
    JsonCreateFailure,
    JsonExtractValueFailure,
    JsonPublishFailure,
    GraphQLError,
}

trait JsonValueExtension {
    fn to_int(&self) -> Result<i64, AppError>;
    fn to_float(&self) -> Result<f64, AppError>;
}

impl JsonValueExtension for serde_json::Value {
    fn to_int(&self) -> Result<i64, AppError> {
        self.as_i64().ok_or(AppError::JsonExtractValueFailure)
    }
    fn to_float(&self) -> Result<f64, AppError> {
        self.as_f64().ok_or(AppError::JsonExtractValueFailure)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> Self {
        AppError::JsonCreateFailure
    }
}

fn to_map(
    json: &serde_json::Value,
) -> Result<&serde_json::Map<String, serde_json::Value>, AppError> {
    match json.as_object() {
        Some(value) => Ok(value),
        None => {
            return Err(AppError::JsonCreateFailure);
        }
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = json!({ "error": "failure" }).to_string();
        let len = body.len();

        Response::build()
            .status(Status::BadRequest)
            .header(ContentType::JSON)
            .sized_body(len, Cursor::new(body))
            .ok()
    }
}

struct GradientVector {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl GradientVector {
    const TOP_LEFT_BOTTOM_RIGHT: GradientVector = Self {
        x1: 0,
        y1: 0,
        x2: 1,
        y2: 1,
    };
}

struct RGB(i32, i32, i32);

impl RGB {
    fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    fn adjust_brightness(&self, percentage: f32) -> Self {
        let ratio = (100.0 + percentage) / 100.0;
        Self(
            (self.0 as f32 * ratio).max(0.0).min(255.0) as i32,
            (self.1 as f32 * ratio).max(0.0).min(255.0) as i32,
            (self.2 as f32 * ratio).max(0.0).min(255.0) as i32,
        )
    }
}

struct GradientColor {
    id: &'static str,
    rgb: RGB,
}

impl GradientColor {
    const BLUE: GradientColor = Self {
        id: "blue",
        rgb: RGB(124, 181, 236),
    };
    const GRAY: GradientColor = Self {
        id: "gray",
        rgb: RGB(67, 67, 72),
    };
    const GREEN: GradientColor = Self {
        id: "green",
        rgb: RGB(144, 237, 125),
    };
    const ORANGE: GradientColor = Self {
        id: "orange",
        rgb: RGB(247, 163, 92),
    };
    const PURPLE: GradientColor = Self {
        id: "purple",
        rgb: RGB(128, 133, 233),
    };
    const PINK: GradientColor = Self {
        id: "pink",
        rgb: RGB(241, 92, 128),
    };
    const YELLOW: GradientColor = Self {
        id: "yellow",
        rgb: RGB(228, 211, 84),
    };
    const CYAN: GradientColor = Self {
        id: "cyan",
        rgb: RGB(43, 144, 143),
    };
    const RED: GradientColor = Self {
        id: "red",
        rgb: RGB(244, 91, 91),
    };
    const TURQUOISE: GradientColor = Self {
        id: "turquoise",
        rgb: RGB(145, 232, 225),
    };

    const ALL_COLORS: [GradientColor; 10] = [
        Self::BLUE,
        Self::GRAY,
        Self::GREEN,
        Self::ORANGE,
        Self::PURPLE,
        Self::PINK,
        Self::YELLOW,
        Self::CYAN,
        Self::RED,
        Self::TURQUOISE,
    ];
}

trait LinearGraditionExtension {
    fn set_gradient_vector(self, gv: &GradientVector) -> Self;
    fn set_gradient_color(self, gc: &GradientColor) -> Self;
}

impl LinearGraditionExtension for LinearGradient {
    fn set_gradient_vector(self, gv: &GradientVector) -> Self {
        self.set("x1", format!("{}", gv.x1))
            .set("y1", format!("{}", gv.y1))
            .set("x2", format!("{}", gv.x2))
            .set("y2", format!("{}", gv.y2))
    }
    fn set_gradient_color(self, gc: &GradientColor) -> Self {
        let from = Stop::new()
            .set("offset", "0%")
            .set("stop-color", gc.rgb.to_hex_string());
        let to = Stop::new().set("offset", "100%").set(
            "stop-color",
            gc.rgb.adjust_brightness(-50.0).to_hex_string(),
        );
        self.add(from).add(to)
    }
}

struct GradientColorManager {
    index: i32,
    length: usize,
}

impl GradientColorManager {
    pub fn new() -> Self {
        Self {
            index: 0,
            length: GradientColor::ALL_COLORS.len(),
        }
    }
    pub fn next(&mut self) -> String {
        let mut i = self.index;
        let color = GradientColor::ALL_COLORS.get(i as usize);
        i += 1;
        if i == self.length as i32 {
            i = 0;
        }
        self.index = i;
        match color {
            Some(c) => c.id.to_string(),
            None => String::from("blue"),
        }
    }
}

fn create_definitions() -> Definitions {
    let mut defs = Definitions::new();
    // TODO for_eachに直してみたい
    for c in &GradientColor::ALL_COLORS {
        let grad = LinearGradient::new()
            .set("id", c.id)
            .set_gradient_vector(&GradientVector::TOP_LEFT_BOTTOM_RIGHT)
            .set_gradient_color(c);
        defs = defs.add(grad);
    }
    defs
}

fn create_svg(data: &Vec<SvgData>, width: i32) -> Result<String, AppError> {
    // 個々の棒グラフの高さを20に固定する。
    let bar_height = 20;

    let view_height = data.len() as i32 * (bar_height + 10);

    // 引数で指定されたwidthを持つSVGを生成する。
    // ただし、高さはデータの数に応じて自動的に決定する。
    let mut document = Document::new().set("viewBox", (0, 0, width, 500));
    let defs = create_definitions();

    document = document.add(defs);

    let sum: f64 = data.iter().map(|d| d.size).sum::<i64>() as f64;
    let data = data
        .into_iter()
        .map(|d| {
            let ratio = d.size as f64 / sum;
            let new_data = SvgData {
                name: d.name.to_string(),
                size: d.size,
                ratio,
            };
            new_data
        })
        .collect::<Vec<SvgData>>();

    let mut y = 0;
    let mut color_manager = GradientColorManager::new();
    for svg_data in data {
        let rect = Rectangle::new()
            .set("x", 100) // テキストの分だけ棒グラフを右に移動
            .set("y", y)
            .set("rx", 5)
            .set("ry", 5)
            .set("width", svg_data.ratio * 200.0)
            .set("height", 20) // 高さを調整
            .set("fill", format!("url(#{})", color_manager.next()));
        document = document.add(rect);

        let text = Text::new()
            .set("x", 0)
            .set("y", y + 15) // テキストを棒グラフの中央に配置
            .add(svg::node::Text::new(format!(
                "{}({})",
                svg_data.name, svg_data.size
            )));
        document = document.add(text);

        y += 30; // 間隔を調整
    }

    // SVG仕様確認用に直接SVGファイル出力しておく。
    // SVGファイルを直接編集して表示を確認するために使う。
    // TODO 後で消す
    let file = File::create("image.svg");
    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            return Err(AppError::JsonPublishFailure);
        }
    };
    let _ = file.write_all(document.to_string().as_bytes());

    Ok(document.to_string())
}

#[derive(Debug)]
struct SvgData {
    name: String,
    size: i64,
    ratio: f64,
}

fn convert_to_svg_data(stats: &ResponseData) -> Result<HashMap<String, SvgData>, AppError> {
    let mut data: HashMap<String, SvgData> = HashMap::new();

    let viewer = &stats.viewer;
    let repositories = viewer
        .repositories
        .edges
        .as_ref()
        .ok_or(AppError::JsonPublishFailure)?;

    for repo in repositories {
        let repo_node = repo
            .as_ref()
            .ok_or(AppError::JsonPublishFailure)?
            .node
            .as_ref()
            .ok_or(AppError::JsonPublishFailure)?;

        let repo_langs = repo_node
            .languages
            .as_ref()
            .ok_or(AppError::JsonPublishFailure)?
            .edges
            .as_ref()
            .ok_or(AppError::JsonPublishFailure)?;

        for repo_lang in repo_langs {
            let repo_lang = repo_lang.as_ref().ok_or(AppError::JsonPublishFailure)?;
            let size = repo_lang.size;
            let name = &repo_lang.node.name;

            if name == "HTML" {
                continue;
            }

            let _color = repo_lang
                .node
                .color
                .as_ref()
                .ok_or(AppError::JsonPublishFailure)?;

            let entry = data.entry(name.to_string()).or_insert(SvgData {
                name: name.to_string(),
                size: 0,
                ratio: 0.0,
            });
            entry.size += size;
        }
    }
    Ok(data)
}

#[derive(Deserialize)]
struct GraphQLResponse {
    data: github_stats::ResponseData,
}

async fn get_github_summary() -> Result<github_stats::ResponseData, Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set");
    let client = Client::builder().user_agent("MyApp/0.1").build()?;
    let query = GithubStats::build_query(query::github_stats::Variables {});

    let response = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .json(&query)
        .send()
        .await?;

    let body_text = response.text().await?;
    let result: GraphQLResponse = serde_json::from_str(&body_text)?;

    Ok(result.data)
}

#[get("/")]
async fn index() -> Result<(ContentType, String), AppError> {
    let github_summary = get_github_summary()
        .await
        .map_err(|_| AppError::GraphQLError)?;

    let data = convert_to_svg_data(&github_summary)?;
    let mut data: Vec<SvgData> = data.into_iter().map(|(_, v)| v).collect();
    data.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());
    data.truncate(10);
    println!("data: {:?}", data);

    let svg_data = create_svg(&data, 300)?;

    Ok((ContentType::SVG, svg_data))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
