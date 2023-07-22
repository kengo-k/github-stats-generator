use crate::config;
use std::collections::HashMap;
use svg::node::element::{Path, Rectangle, Style, Text};
use svg::Document;

use crate::graphql::RepositoryStat;

const CSS: &'static str = r#" .top_lang_chart text {
    font: 400 11px 'Segoe UI', Ubuntu, Sans-Serif;
}
.title {
    font: 600 18px 'Segoe UI', Ubuntu, Sans-Serif;
    fill: #2f80ed;
}
.star path {
    fill: #4c71f2;
}
.star text {
    font: 600 14px 'Segoe UI', Ubuntu, "Helvetica Neue", Sans-Serif;
    fill: #434d58;
}"#;

const STAR_ICON: &'static str = "M8 .25a.75.75 0 0 1 .673.418l1.882 3.815 4.21.612a.75.75 0 0 1 .416 1.279l-3.046 2.97.719 4.192a.751.751 0 0 1-1.088.791L8 12.347l-3.766 1.98a.75.75 0 0 1-1.088-.79l.72-4.194L.818 6.374a.75.75 0 0 1 .416-1.28l4.21-.611L7.327.668A.75.75 0 0 1 8 .25Zm0 2.445L6.615 5.5a.75.75 0 0 1-.564.41l-3.097.45 2.24 2.184a.75.75 0 0 1 .216.664l-.528 3.084 2.769-1.456a.75.75 0 0 1 .698 0l2.77 1.456-.53-3.084a.75.75 0 0 1 .216-.664l2.24-2.183-3.096-.45a.75.75 0 0 1-.564-.41L8 2.694Z";

fn create_star_icon(count: i64) -> Document {
    let mut root = Document::new().set("class", "star");

    let text = Text::new()
        .set("x", 25)
        .set("y", 13)
        .set("width", 100)
        .add(svg::node::Text::new(format!(
            "Total Stars Earned: {}",
            count
        )));

    let path = Path::new().set("d", STAR_ICON);
    let star = Document::new()
        .set("viewBox", "0 0 16 16")
        .set("width", 16)
        .set("height", 16)
        .add(path);

    root = root.add(text).add(star);
    root
}

#[derive(Debug)]
pub struct LanguageSummary {
    pub total_size: i64,
    pub data: HashMap<String, LanguageSummaryValue>,
}

#[derive(Debug)]
pub struct LanguageSummaryValue {
    pub name: String,
    pub color: String,
    pub size: i64,
}

impl LanguageSummary {
    pub fn new() -> Self {
        let data: HashMap<String, LanguageSummaryValue> = HashMap::new();
        Self {
            total_size: 0,
            data,
        }
    }
}

#[derive(Debug)]
pub struct Renderer {
    pub stats: Vec<RepositoryStat>,
    pub language_summary: LanguageSummary,
}

impl Renderer {
    pub fn new(stats: Vec<RepositoryStat>) -> Self {
        let config = config::load();
        let mut language_summary = LanguageSummary::new();
        let map = &mut language_summary.data;
        for s in &stats {
            if config.ignore_repositories.contains(&s.name) {
                continue;
            }
            let ls = &s.languages;
            for l in ls {
                if config.ignore_languages.contains(&l.name) {
                    continue;
                }
                language_summary.total_size += l.size;
                let mut entry = map.entry(l.name.clone()).or_insert(LanguageSummaryValue {
                    name: l.name.clone(),
                    color: l.color.clone(),
                    size: 0,
                });
                (*entry).size += l.size;
            }
        }
        Self {
            stats,
            language_summary,
        }
    }

    pub fn render(&self) -> Document {
        let styles = Style::new(CSS);
        let all_star_count = self
            .stats
            .iter()
            .map(|item| item.stargazer_count)
            .sum::<i64>();
        let star = create_star_icon(all_star_count).set("x", 20).set("y", 10);
        let top_lang_charts = self.create_top_lang_charts();
        let root = Document::new()
            .set("width", 300)
            .set("height", 500)
            .add(styles)
            .add(star)
            .add(top_lang_charts);

        root
    }

    fn create_top_lang_charts(&self) -> Document {
        let config = config::load();
        let values: Vec<_> = self.language_summary.data.values().collect();
        let height = values.len() * 35;
        let mut root = Document::new();
        let mut top_lang_chart = Document::new()
            .set("x", 0)
            .set("y", 50)
            .set("width", 300)
            .set("height", height)
            .set("viewBox", (0, 0, 300, height))
            .set("class", "top_lang_chart");
        let mut values: Vec<_> = self.language_summary.data.values().collect();
        values.sort_by(|a, b| (*b).size.partial_cmp(&a.size).unwrap());
        values.truncate(config.languages_count);

        let charts: Vec<_> = values
            .into_iter()
            .enumerate()
            .map(|(i, d)| {
                let text = format!(
                    "{}: {:.1}% ({}KB)",
                    d.name.as_str(),
                    d.size as f64 / self.language_summary.total_size as f64 * 100.0,
                    d.size / 1000
                );
                let doc = create_bar_chart(
                    text.as_str(),
                    d.size as f64 / self.language_summary.total_size as f64 * 100.0,
                    d.color.as_str(),
                );
                doc.set("y", i * 40)
            })
            .collect::<Vec<_>>();

        for chart in charts {
            top_lang_chart = top_lang_chart.add(chart)
        }

        let title = Text::new()
            .set("x", 0)
            .set("y", 30)
            .set("class", "title")
            .add(svg::node::Text::new("Most Used Languages"));

        root = root
            .set("x", 20)
            .set("y", 30)
            .add(title)
            .add(top_lang_chart);
        root
    }
}

pub fn create_bar_chart(lang_name: &str, ratio: f64, color: &str) -> Document {
    const BAR_TOP: f32 = 27.5;
    const BAR_HEIGHT: i32 = 8;
    const BAR_ROUND: i32 = 5;
    let mut document = Document::new().set("width", 250);
    let text = Text::new()
        .set("x", 0)
        .set("y", 20)
        .add(svg::node::Text::new(format!("{}", lang_name)));
    let whole_rect = Rectangle::new()
        .set("x", 0)
        .set("y", BAR_TOP)
        .set("rx", BAR_ROUND)
        .set("ry", BAR_ROUND)
        .set("width", 205)
        .set("height", BAR_HEIGHT)
        .set("fill", "#ddd")
        .set("class", "whole");
    let ratio_rect = Rectangle::new()
        .set("x", 0)
        .set("y", BAR_TOP)
        .set("rx", BAR_ROUND)
        .set("ry", BAR_ROUND)
        .set("width", format!("{}%", ratio))
        .set("height", BAR_HEIGHT)
        .set("fill", color)
        .set("class", "ratio");

    document = document.add(text).add(whole_rect).add(ratio_rect);

    document
}

#[cfg(test)]
mod tests {
    use super::*;
    use sxd_document::{parser, Package};
    use sxd_xpath::{Context, Factory};

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

        let source = create_bar_chart("rust", 9.37, "red").to_string();
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
