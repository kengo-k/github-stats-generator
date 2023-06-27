#[macro_use] extern crate rocket;

use rocket::http::ContentType;

#[get("/")]
fn index() -> (ContentType, String) {
    // 例として、SVGデータを動的に生成する関数を呼び出します。
    let svg_data = generate_svg_data();

    // 生成されたSVGデータを返す。
    (ContentType::SVG, svg_data)
}

fn generate_svg_data() -> String {
    // 実際にはここでAPI呼び出しを行い、その結果からSVGデータを生成します。
    // ここでは単純化のために固定の文字列を返すものとします。
    "<svg version=\"1.1\" baseProfile=\"full\" width=\"300\" height=\"200\" xmlns=\"http://www.w3.org/2000/svg\"><rect width=\"100%\" height=\"100%\" fill=\"red\"/><circle cx=\"150\" cy=\"100\" r=\"80\" fill=\"green\"/><text x=\"150\" y=\"125\" font-size=\"60\" text-anchor=\"middle\" fill=\"white\">SVG</text></svg>".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
