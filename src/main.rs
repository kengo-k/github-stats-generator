mod fetch;
mod generated;

use generated::query::github_stats::ResponseData;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::string::ToString;
use svg::node::element::{Rectangle, Text};
use svg::Document;

#[derive(Debug)]
enum AppError {
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

fn create_svg(data: &Vec<SvgData>, width: i32) -> Result<String, AppError> {
    // 個々の棒グラフの高さを20に固定する。
    let bar_height = 20;

    let view_height = data.len() as i32 * (bar_height + 10);

    // 引数で指定されたwidthを持つSVGを生成する。
    // ただし、高さはデータの数に応じて自動的に決定する。
    let mut document = Document::new().set("viewBox", (0, 0, width, 500));

    let sum: f64 = data.iter().map(|d| d.size).sum::<i64>() as f64;
    let data = data
        .into_iter()
        .map(|d| {
            let ratio = d.size as f64 / sum;
            let new_data = SvgData {
                name: d.name.to_string(),
                size: d.size,
                color: d.color.to_string(),
                ratio,
            };
            new_data
        })
        .collect::<Vec<SvgData>>();

    let mut y = 0;
    for svg_data in data {
        let rect = Rectangle::new()
            .set("x", 100) // テキストの分だけ棒グラフを右に移動
            .set("y", y)
            .set("rx", 5)
            .set("ry", 5)
            .set("width", svg_data.ratio * 200.0)
            .set("height", 20) // 高さを調整
            .set("fill", format!("{}", svg_data.color));
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

    Ok(document.to_string())
}

#[derive(Debug)]
struct SvgData {
    name: String,
    size: i64,
    ratio: f64,
    color: String,
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

            let color = repo_lang
                .node
                .color
                .as_ref()
                .ok_or(AppError::JsonPublishFailure)?;

            let entry = data.entry(name.to_string()).or_insert(SvgData {
                name: name.to_string(),
                size: 0,
                ratio: 0.0,
                color: color.to_string(),
            });
            entry.size += size;
        }
    }
    Ok(data)
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let github_summary = fetch::get_github_summary()
        .await
        .map_err(|_| AppError::GraphQLError)?;

    let data = convert_to_svg_data(&github_summary)?;
    let mut data: Vec<SvgData> = data.into_iter().map(|(_, v)| v).collect();
    data.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());
    data.truncate(10);

    let svg_data = create_svg(&data, 300)?;
    let file = File::create("image.svg");
    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            return Err(AppError::JsonPublishFailure);
        }
    };
    let _ = file.write_all(svg_data.as_bytes());

    Ok(())
}
