use crate::AppError;
use svg::node::element::{Rectangle, Style, Text};
use svg::Document;

const CSS: &'static str = r#" text {
    font: 400 11px 'Segoe UI', Ubuntu, Sans-Serif;
}"#;

fn create_bar_chart(lang_name: &str, size: i64, ratio: f64, color: &str) -> Document {
    const LEFT: i32 = 10;
    const BAR_TOP: f32 = 27.5;
    const BAR_HEIGHT: i32 = 8;
    const BAR_ROUND: i32 = 5;
    let mut document = Document::new()
        .set("width", 250);
    let text = Text::new()
        .set("x", LEFT)
        .set("y", 20)
        .add(svg::node::Text::new(format!("{}", lang_name)));
    let whole_rect = Rectangle::new()
        .set("x", LEFT)
        .set("y", BAR_TOP)
        .set("rx", BAR_ROUND)
        .set("ry", BAR_ROUND)
        .set("width", 205)
        .set("height", BAR_HEIGHT)
        .set("fill", "#ddd")
        .set("class", "whole")
        ;
    let ratio_rect = Rectangle::new()
        .set("x", LEFT)
        .set("y", BAR_TOP)
        .set("rx", BAR_ROUND)
        .set("ry", BAR_ROUND)
        .set("width", format!("{}%", ratio))
        .set("height", BAR_HEIGHT)
        .set("fill", color)
        .set("class", "ratio")
        ;

    document = document.add(text).add(whole_rect).add(ratio_rect);

    document
}

pub fn write(data: &Vec<crate::convert::SvgData>) -> Result<String, AppError> {
    let style = Style::new(CSS);
    let sum = &data.iter().map(|d|d.size).sum::<i64>();
    let bars: Vec<_> = data
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let ratio = d.size as f64 / *sum as f64 * 100.0;
            let text = format!("{}: {}% ({}KB)", d.name.as_str(), ratio as i32, d.size / 1000);
            let doc = create_bar_chart(text.as_str(), d.size, ratio, d.color.as_str());
            doc.set("x", 10).set("y", i * 35)
        })
        .collect::<Vec<_>>()
        ;
    //let bar = create_bar_chart("HTML", 100, 59.56, "red");
    let mut root = Document::new()
        .set("width", 300)
        .set("height", 285)
        .set("viewBox", (0, 0, 300, 285))
        .add(style);

    for bar in bars {
        root = root.add(bar);
    }

    Ok(root.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sxd_document::{dom, parser, Package};
    use sxd_xpath::{evaluate_xpath, Context, Error, Factory, Value, XPath};

    struct DocumentWrapper {
        package: Package,
        ns: Option<(String, String)>,
    }

    impl DocumentWrapper {
        pub fn new(xml: &str) -> Self {
            let package = parser::parse(xml).unwrap();
            Self { package, ns: None }
        }

        pub fn set_namespace(mut self, prefix: &str, namespace: &str) -> Self {
            self.ns = Some((prefix.to_string(), namespace.to_string()));
            self
        }

        pub fn string(&self, xpath_str: &str) -> String {
            let document = self.package.as_document();
            let factory = Factory::new();
            let xpath = factory.build(xpath_str).unwrap().unwrap();
            let mut context = Context::new();
            if let Some(ns) = &self.ns {
                context.set_namespace(&ns.0, &ns.1);
            }
            let value = xpath.evaluate(&context, document.root()).unwrap();
            value.string()
        }
    }

    #[test]
    fn test_xpath() {
        struct Test {
            xpath: String,
            source: String,
            expected: String,
            ns: Option<(String, String)>,
        }

        impl Test {
            pub fn new(xpath: &str, source: &str, expected: &str) -> Self {
                Self {
                    xpath: xpath.to_string(),
                    source: source.to_string(),
                    expected: expected.to_string(),
                    ns: None,
                }
            }
            pub fn set_namespace(mut self, prefix: &str, namespace: &str) -> Self {
                self.ns = Some((prefix.to_string(), namespace.to_string()));
                self
            }
        }

        let tests = [
            Test::new("/test", "<test>hello</test>", "hello"),
            Test::new("/test/@id", r#"<test id="xxx">hello</test>"#, "xxx"),
            Test::new("/*/a + /*/b", "<test><a>1</a><b>2</b></test>", "3"),
            Test::new(
                "/ns:svg/@viewBox",
                r#"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg"></svg>"#,
                "0 0 100 100",
            )
            .set_namespace("ns", "http://www.w3.org/2000/svg"),
        ];

        for t in tests {
            let mut document = DocumentWrapper::new(t.source.as_str());
            if let Some(ns) = &t.ns {
                document = document.set_namespace(&ns.0, &ns.1);
            }
            let value = document.string(t.xpath.as_str());
            assert_eq!(value, t.expected.as_str());
        }
    }

    #[test]
    fn test_create_bar_chart() {
        struct Test {
            xpath: String,
            expected: String,
        }

        impl Test {
            pub fn new(xpath: &str, expected: &str) -> Self {
                Self {
                    xpath: xpath.to_string(),
                    expected: expected.to_string(),
                }
            }
        }

        let source = create_bar_chart("rust", 100, 9.37, "red").to_string();
        let doc =
            DocumentWrapper::new(source.as_str()).set_namespace("ns", "http://www.w3.org/2000/svg");

        let tests = [
            Test::new("/ns:svg/ns:text", "rust"),
            Test::new("/ns:svg/ns:text/text()", "rust"),
            Test::new("/ns:svg/ns:rect[@class='whole']/@fill", "#ddd"),
            Test::new("/ns:svg/ns:rect[@class='ratio']/@fill", "red"),
        ];

        for t in tests {
            let expected = t.expected;
            let actual = doc.string(t.xpath.as_str());
            assert_eq!(actual.trim(), expected);
        }
    }
}
