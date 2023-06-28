#[macro_use] extern crate rocket;

use rocket::http::ContentType;

use svg::Document;
use svg::node::element::{Rectangle, Text};

fn create_bar_chart(data: &str, width: i32, height: i32) -> String {
    let json: serde_json::Value = serde_json::from_str(data).unwrap();
    let mut document = Document::new()
        .set("viewBox", (0, 0, 500, 500));

    let mut y = 0;
    for (language, size) in json.as_object().unwrap() {
        let rect = Rectangle::new()
            .set("x", 100)  // テキストの分だけ棒グラフを右に移動
            .set("y", y)
            .set("width", size.as_i64().unwrap() / 10)
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


#[get("/")]
fn index() -> (ContentType, String) {
    let data = r#"{ "Java": 1000, "TypeScript": 200 }"#;
    // 例として、SVGデータを動的に生成する関数を呼び出します。
    let svg_data = create_bar_chart(data, 1000, 1000);

    // 生成されたSVGデータを返す。
    (ContentType::SVG, svg_data)
}

// fn generate_svg_data() -> String {
//     // 実際にはここでAPI呼び出しを行い、その結果からSVGデータを生成します。
//     // ここでは単純化のために固定の文字列を返すものとします。
//     "<svg version=\"1.1\" baseProfile=\"full\" width=\"300\" height=\"200\" xmlns=\"http://www.w3.org/2000/svg\"><rect width=\"100%\" height=\"100%\" fill=\"red\"/><circle cx=\"150\" cy=\"100\" r=\"80\" fill=\"green\"/><text x=\"150\" y=\"125\" font-size=\"60\" text-anchor=\"middle\" fill=\"white\">SVG</text></svg>".to_string()
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
