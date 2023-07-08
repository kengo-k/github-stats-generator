#[macro_use] extern crate rocket;

use rocket::http::{ContentType, Status};

use svg::Document;
use svg::node::element::{Definitions, LinearGradient, Rectangle, Stop, Text};
use std::fs::File;
use std::io::Cursor;
use std::io::prelude::*;
use rocket::{Request, response, Response};
use rocket::response::Responder;
use serde_json::json;

#[derive(Debug)]
enum AppError {
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

fn create_bar_chart(data: &str, width: i32) -> Result<String, AppError> {

    let json: serde_json::Value = serde_json::from_str(data)?;
    let json_map = to_map(&json)?;

    // 個々の棒グラフの高さを20に固定する。
    let bar_height = 20;

    let view_height = json_map.len() as i32 * (bar_height + 10);

    // 引数で指定されたwidthを持つSVGを生成する。
    // ただし、高さはデータの数に応じて自動的に決定する。
    let mut document = Document::new()
        .set("viewBox", (0, 0, width, view_height));

    let mut defs = Definitions::new();
    let mut grad = LinearGradient::new()
        .set("id", "grad1")
        .set("x1", "0")
        .set("x2", "0")
        .set("y1", "0")
        .set("y2", "1");

    let stop = Stop::new()
        .set("offset", "0%")
        .set("stop-color", "white");

    grad = grad.add(stop);

    let stop = Stop::new()
        .set("offset", "100%")
        .set("stop-color", "black");
    grad = grad.add(stop);


    defs = defs.add(grad);

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
            .set("width", size.to_float()? as f64 * 200.0)
            .set("height", 20)  // 高さを調整
            .set("fill", "url(#grad1)");
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
fn get_json() -> String {
    r#"{
        "typescript": 600,
        "go": 300,
        "rust": 50,
        "css": 100,
        "html": 200,
        "elm": 20,
        "yaml": 30,
        "haskell": 20,
        "ocaml": 10,
        "python": 900
    }"#.to_string()
}

#[get("/")]
fn index() -> Result<(ContentType, String), AppError> {
    let data = &get_json();
    // 例として、SVGデータを動的に生成する関数を呼び出します。
    let svg_data = create_bar_chart(data, 300)?;

    // 生成されたSVGデータを返す。
    Ok((ContentType::SVG, svg_data))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
