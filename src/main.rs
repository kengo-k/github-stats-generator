#[macro_use] extern crate rocket;

use rocket::http::ContentType;

use svg::Document;
use svg::node::element::{Rectangle, Text};

fn create_bar_chart(data: &str, width: i32) -> String {
    let json: serde_json::Value = serde_json::from_str(data).unwrap();

    // 個々の棒グラフの高さを20に固定する。
    let bar_height = 20;

    let view_height = json.as_object().unwrap().len() as i32 * bar_height + 200;
    println!("view_height: {}", view_height);

    // 引数で指定されたwidthを持つSVGを生成する。
    // ただし、高さはデータの数に応じて自動的に決定する。
    let mut document = Document::new()
        .set("viewBox", (0, 0, width, view_height));

    // jsonの横幅を加工した新しいjsonデータratio_jsonを作成する
    // ratio_jsonの各要素の値はjsonの全要素の値の合計に対する割合となる
    let mut ratio_json = serde_json::Map::new();
    let mut sum = 0;
    for (_, size) in json.as_object().unwrap() {
        sum += size.as_i64().unwrap();
    }
    for (language, size) in json.as_object().unwrap() {
        let ratio = size.as_i64().unwrap() as f64 / sum as f64;
        ratio_json.insert(language.to_string(), serde_json::Value::from(ratio));
    }
    // Mapで作成したratio_jsonをserde_json::Valueに変換する
    let ratio_json = serde_json::Value::from(ratio_json);
    println!("{:?}", ratio_json);

    let mut y = 0;
    for (language, size) in ratio_json.as_object().unwrap() {
        let rect = Rectangle::new()
            .set("x", 100)  // テキストの分だけ棒グラフを右に移動
            .set("y", y)
            .set("width", size.as_f64().unwrap() * 100.0)
            .set("height", 20)  // 高さを調整
            .set("fill", "blue");
        document = document.add(rect);

        let text = Text::new()
            .set("x", 0)
            .set("y", y + 15)  // テキストを棒グラフの中央に配置
            .add(svg::node::Text::new(language));
        document = document.add(text);

        y += 30;  // 間隔を調整
    }

    document.to_string()
}

/// データを取得する関数
/// TODO: あとでGitHubのGraphQL APIを呼び出して実際のデータを取得する
///
/// 現状では暫定値として以下のキーと値をもつJSON文字列を返す。
///
/// "java": 1000,
/// "typescript": 200,
/// "go": 300,
/// "rust": 50
fn get_json() -> String {
    r#"{
        "java": 1000,
        "typescript": 200,
        "go": 300,
        "rust": 50
    }"#.to_string()
}

#[get("/")]
fn index() -> (ContentType, String) {
    let data = &get_json();
    // 例として、SVGデータを動的に生成する関数を呼び出します。
    let svg_data = create_bar_chart(data, 1000);

    // 生成されたSVGデータを返す。
    (ContentType::SVG, svg_data)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
