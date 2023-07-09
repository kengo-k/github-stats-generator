#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket::http::{ContentType, Status};

use svg::Document;
use svg::node::element::{Definitions, LinearGradient, Rectangle, Stop, Text};
use std::fs::File;
use std::io::Cursor;
use std::io::prelude::*;
use std::string::ToString;
use rocket::{Request, response, Response};
use rocket::response::Responder;
use serde_json::json;
use svg::node::element::path::Command::Line;
use svg::node::element::tag::LinearGradient;

#[derive(Debug)]
enum AppError {
    GetJsonSourceError, // SVG生成の元になる入力データの取得に失敗したことを示すエラー
    JsonCreateFailure,
    JsonExtractValueFailure,
    JsonPublishFailure
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

fn to_map(json: &serde_json::Value) -> Result<&serde_json::Map<String, serde_json::Value>, AppError> {
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

fn create_linear_gradient() -> LinearGradient {
    let grad = LinearGradient::new()
        .set("id", "grad1")
        .set("x1", "0")
        .set("x2", "0")
        .set("y1", "0")
        .set("y2", "1");
    grad
}

struct GradientVector {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

impl GradientVector {
    const TOP_LEFT_BOTTOM_RIGHT: GradientVector = Self { x1: 0, y1: 0, x2: 0, y2: 1 };
}

struct RGB(i32, i32, i32);

impl RGB {

    fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    fn adjust_brightness(&self, percentage: f32) -> Self {
        let ratio = (100.0 + percentage) / 100.0;
        Self((self.0 as f32 * ratio).max(0.0).min(255.0) as i32,
             (self.1 as f32 * ratio).max(0.0).min(255.0) as i32,
             (self.2 as f32 * ratio).max(0.0).min(255.0) as i32)
    }
}

struct GradientColor {
    id: &'static str,
    rgb: RGB
}

impl GradientColor {
    const BLUE: GradientColor = Self { id: "blue", rgb: RGB(124, 181, 236) };
    const GRAY: GradientColor = Self { id: "gray", rgb: RGB(67, 67, 72) };
    const GREEN: GradientColor = Self { id: "green", rgb: RGB( 144, 237, 125) };
    const ORANGE: GradientColor = Self { id: "orange", rgb: RGB(247, 163, 92) };
    const PURPLE: GradientColor = Self { id: "purple", rgb: RGB(128, 133, 233) };
    const PINK: GradientColor = Self { id: "pink", rgb: RGB(241, 92, 128) };
    const YELLOW: GradientColor = Self { id: "yellow", rgb: RGB(228, 211, 84) };
    const CYAN: GradientColor = Self { id: "cyan", rgb: RGB(43, 144, 143) };
    const RED: GradientColor = Self { id: "red", rgb: RGB(244, 91, 91) };
    const TURQUOISE: GradientColor = Self { id: "turquoise", rgb: RGB(145, 232, 225) };
}

trait LinearGraditionExtension {
    fn set_gradient_vector(self, gv: &GradientVector) -> Self;
    fn set_gradient_color(self, gc: &GradientColor) -> Self;
}

impl LinearGraditionExtension for LinearGradient {
    fn set_gradient_vector(self, gv: &GradientVector) -> Self {
        self
            .set("x1", format!("{}", gv.x1))
            .set("y1", format!("{}", gv.y1))
            .set("x2", format!("{}", gv.x2))
            .set("y2", format!("{}", gv.y2))
    }
    fn set_gradient_color(self, gc: &GradientColor) -> Self {
        let from = Stop::new()
            .set("offset", "0%")
            .set("stop-color", gc.rgb.to_hex_string());
        let to = Stop::new()
            .set("offset", "100%")
            .set("stop-color", gc.rgb.adjust_brightness(80.0).to_hex_string());
        self
            .add(from)
            .add(to)
    }
}

struct GradientManager {
    liner_grad: LinearGradient
}

impl GradientManager {
    pub fn new() -> Self {
        Self {
            liner_grad: LinearGradient::new()
        }
    }
}

fn create_definitions() -> Definitions {
    let mut defs = Definitions::new();
    let colors = [GradientColor::BLUE, GradientColor::GRAY, GradientColor::GREEN, GradientColor::ORANGE, GradientColor::PURPLE, GradientColor::PINK, GradientColor::YELLOW, GradientColor::CYAN, GradientColor::RED, GradientColor::TURQUOISE];
    // TODO for_eachに直してみたい
    for c in &colors {
        let grad = LinearGradient::new()
            .set("id", c.id)
            .set_gradient_vector(&GradientVector::TOP_LEFT_BOTTOM_RIGHT)
            .set_gradient_color(c);
        defs = defs.add(grad);
    }
    defs
}

fn create_bar_chart(data: &str, width: i32) -> Result<String, AppError> {

    let json: serde_json::Value = serde_json::from_str(data)?;
    let json_map = to_map(&json)?;

    // 個々の棒グラフの高さを20に固定する。
    let bar_height = 10;

    let view_height = json_map.len() as i32 * (bar_height + 10);

    // 引数で指定されたwidthを持つSVGを生成する。
    // ただし、高さはデータの数に応じて自動的に決定する。
    let mut document = Document::new()
        .set("viewBox", (0, 0, width, 500));
    let defs = create_definitions();

    document = document.add(defs);

    // jsonの横幅を加工した新しいjsonデータratio_jsonを作成する
    // ratio_jsonの各要素の値はjsonの全要素の値の合計に対する割合となる
    let mut ratio_json = serde_json::Map::new();

    let mut sum = 0;
    for (_, size) in json_map {
        sum += size.to_int()?
    }

    for (language, size) in json_map {
        let ratio = size.to_int()? as f64 / sum as f64;
        ratio_json.insert(language.to_string(), serde_json::Value::from(ratio));
    }
    // Mapで作成したratio_jsonをserde_json::Valueに変換する
    let ratio_json = serde_json::Value::from(ratio_json);
    let ration_json_map = to_map(&ratio_json)?;

    let mut y = 0;
    for (language, size) in ration_json_map {
        let rect = Rectangle::new()
            .set("x", 100)  // テキストの分だけ棒グラフを右に移動
            .set("y", y)
            .set("rx", 1)
            .set("ry", 1)
            .set("width", size.to_float()? as f64 * 200.0)
            .set("height", 200)  // 高さを調整
            .set("fill", "url(#red)");
        document = document.add(rect);

        let text = Text::new()
            .set("x", 0)
            .set("y", y + 15)  // テキストを棒グラフの中央に配置
            .add(svg::node::Text::new(language));
        document = document.add(text);

        y += 30;  // 間隔を調整
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

/// データを取得する関数
/// TODO: あとでGitHubのGraphQL APIを呼び出して実際のデータを取得する
fn get_json() -> Result<String, AppError> {
    let mut data = HashMap::new();
    data.insert("typescript", 600);
    let result = serde_json::to_string(&data);
    result.map_err(|_| AppError::GetJsonSourceError)
}

#[get("/")]
fn index() -> Result<(ContentType, String), AppError> {
    let data = &get_json()?;
    // 例として、SVGデータを動的に生成する関数を呼び出します。
    let svg_data = create_bar_chart(data, 300)?;

    // 生成されたSVGデータを返す。
    Ok((ContentType::SVG, svg_data))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
