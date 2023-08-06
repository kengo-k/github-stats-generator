use crate::config;
use chrono::Utc;
use log::debug;
use std::collections::HashMap;
use svg::node::element::{Definitions, LinearGradient, Link, Path, Rectangle, Stop, Style, Text};
use svg::Document;

use crate::graphql::RepositoryStat;

const CSS: &'static str = r#".chart text {
    font: 400 9px 'Segoe UI', Ubuntu, Sans-Serif;
}
.title {
    font: 600 11px 'Segoe UI', Ubuntu, Sans-Serif;
    fill: #2f80ed;
}
.star path {
    fill: #4c71f2;
}
.star text {
    font: 600 14px 'Segoe UI', Ubuntu, "Helvetica Neue", Sans-Serif;
    fill: #434d58;
}
.footer > text {
    font: 200 11px 'Segoe UI', Ubuntu, Sans-Serif;
    fill: gray;
}
.footer > a > text {
    font: 200 11px 'Segoe UI', Ubuntu, Sans-Serif;
    fill: gray;
    text-decoration: underline;
}"#;

const STAR_ICON: &'static str = "M8 .25a.75.75 0 0 1 .673.418l1.882 3.815 4.21.612a.75.75 0 0 1 .416 1.279l-3.046 2.97.719 4.192a.751.751 0 0 1-1.088.791L8 12.347l-3.766 1.98a.75.75 0 0 1-1.088-.79l.72-4.194L.818 6.374a.75.75 0 0 1 .416-1.28l4.21-.611L7.327.668A.75.75 0 0 1 8 .25Zm0 2.445L6.615 5.5a.75.75 0 0 1-.564.41l-3.097.45 2.24 2.184a.75.75 0 0 1 .216.664l-.528 3.084 2.769-1.456a.75.75 0 0 1 .698 0l2.77 1.456-.53-3.084a.75.75 0 0 1 .216-.664l2.24-2.183-3.096-.45a.75.75 0 0 1-.564-.41L8 2.694Z";
const CHART_WIDTH: i32 = 200;

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

#[derive(Debug)]
pub struct RepositorySummary {
    pub total_commit_count: i64,
    pub total_active_commit_count: i64,
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

impl RepositorySummary {
    pub fn new() -> Self {
        Self {
            total_commit_count: 0,
            total_active_commit_count: 0,
        }
    }
}

#[derive(Debug)]
pub struct Renderer {
    pub stats: Vec<RepositoryStat>,
    pub language_summary: LanguageSummary,
    pub language_colors: HashMap<String, String>,
    pub repository_summary: RepositorySummary,
}

impl Renderer {
    pub fn new(stats: Vec<RepositoryStat>, language_colors: HashMap<String, String>) -> Self {
        let config = config::load();
        let mut language_summary = LanguageSummary::new();
        let mut repository_summary = RepositorySummary::new();
        let map = &mut language_summary.data;
        let stats: Vec<_> = stats
            .into_iter()
            .filter(|s| !config.ignore_repositories.contains(&s.name))
            .collect();
        for s in &stats {
            debug!("{}, {}", s.name, s.period_commit_count);
            repository_summary.total_commit_count += s.total_commit_count;
            repository_summary.total_active_commit_count += s.period_commit_count;
            let ls = &s.languages;
            for l in ls {
                if config.ignore_languages.contains(&l.name) {
                    continue;
                }
                language_summary.total_size += l.size;
                let mapped_lang = config.language_mapping.get(&l.name);
                let lang_name = match mapped_lang {
                    Some(name) => name,
                    None => &l.name,
                };
                let color = language_colors.get(lang_name);
                let color = match color {
                    Some(c) => c,
                    None => &l.color,
                };
                let renamed = config.rename_language.get(lang_name);
                let renamed = match renamed {
                    Some(name) => name,
                    None => lang_name,
                };
                let mut entry = map
                    .entry(renamed.to_string())
                    .or_insert(LanguageSummaryValue {
                        name: renamed.to_string(),
                        color: color.clone(),
                        size: 0,
                    });
                (*entry).size += l.size;
            }
        }
        debug!("repository_summary: {:?}", repository_summary);
        Self {
            stats,
            language_summary,
            language_colors,
            repository_summary,
        }
    }

    pub fn render(&mut self) -> Document {
        let styles = Style::new(CSS);
        let star_count = self
            .stats
            .iter()
            .map(|item| item.stargazer_count)
            .sum::<i64>();
        let header_pane = create_header_pane(star_count, 20, 10);
        let top_langs_chart = self.create_top_langs_chart(20, 30);
        let top_commits_chart = self.create_top_commits_chart(240, 30);
        let top_active_commits_chart = self.create_top_active_commits_chart(460, 30);
        let footer_pane = create_footer_pane(20, 500);

        let defs = Definitions::new()
            .add(create_gradient("green-grad", "#66ff66", "#009900"))
            .add(create_gradient("blue-grad", "#66ccff", "#0000ff"));

        let root = Document::new()
            .set("width", 660)
            .set("height", 540)
            .set("viewBox", "0 0 660 540")
            .add(styles)
            .add(defs)
            .add(header_pane)
            .add(top_langs_chart)
            .add(top_commits_chart)
            .add(top_active_commits_chart)
            .add(footer_pane);

        root
    }

    fn create_top_langs_chart(&self, x: i32, y: i32) -> Document {
        let config = config::load();
        let mut root = Document::new();
        let mut chart = Document::new()
            .set("x", 0)
            .set("y", 50)
            .set("class", "chart");
        let mut values: Vec<_> = self.language_summary.data.values().collect();
        values.sort_by(|a, b| (*b).size.partial_cmp(&a.size).unwrap());
        values.truncate(config.languages_count);

        let bars: Vec<_> = values
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

        for bar in bars {
            chart = chart.add(bar)
        }

        let title = create_chart_title("Top Languages", 0, 30);
        root = root.set("x", x).set("y", y).add(title).add(chart);
        root
    }

    fn create_top_commits_chart(&self, x: i32, y: i32) -> Document {
        let config = config::load();
        let mut root = Document::new();
        let mut chart = Document::new()
            .set("x", 0)
            .set("y", 50)
            .set("class", "chart");
        let mut values = self.stats.clone();
        values = values.into_iter().filter(|item| !item.is_private).collect();
        values.sort_by(|a, b| {
            (*b).total_commit_count
                .partial_cmp(&a.total_commit_count)
                .unwrap()
        });
        values.truncate(config.languages_count);
        let bars: Vec<_> = values
            .into_iter()
            .enumerate()
            .map(|(i, r)| {
                let text = format!(
                    "{}: {:.1}% ({})",
                    r.name,
                    r.total_commit_count as f64 / self.repository_summary.total_commit_count as f64
                        * 100.0,
                    r.total_commit_count
                );
                let doc = create_bar_chart(
                    text.as_str(),
                    r.total_commit_count as f64 / self.repository_summary.total_commit_count as f64
                        * 100.0,
                    "url(#blue-grad)",
                );
                doc.set("y", i * 40)
            })
            .collect::<Vec<_>>();

        for bar in bars {
            chart = chart.add(bar)
        }

        let title = create_chart_title("Top Commits", 0, 30);
        root = root.set("x", x).set("y", y).add(title).add(chart);
        root
    }

    fn create_top_active_commits_chart(&self, x: i32, y: i32) -> Document {
        let config = config::load();
        let mut root = Document::new();
        let mut chart = Document::new()
            .set("x", 0)
            .set("y", 50)
            .set("class", "chart");
        let mut values = self.stats.clone();
        values = values
            .into_iter()
            .filter(|item| item.period_commit_count > 0 && !item.is_private)
            .collect();
        values.sort_by(|a, b| {
            (*b).period_commit_count
                .partial_cmp(&a.period_commit_count)
                .unwrap()
        });
        values.truncate(config.languages_count);
        let bars: Vec<_> = values
            .into_iter()
            .enumerate()
            .map(|(i, r)| {
                let text = format!(
                    "{}: {:.1}% ({})",
                    r.name,
                    r.period_commit_count as f64
                        / self.repository_summary.total_active_commit_count as f64
                        * 100.0,
                    r.period_commit_count
                );
                let doc = create_bar_chart(
                    text.as_str(),
                    r.period_commit_count as f64
                        / self.repository_summary.total_active_commit_count as f64
                        * 100.0,
                    "url(#green-grad)",
                );
                doc.set("y", i * 40)
            })
            .collect::<Vec<_>>();

        for bar in bars {
            chart = chart.add(bar)
        }

        let title = create_chart_title("Top Active Commits(1week)", 0, 30);
        root = root.set("x", x).set("y", y).add(title).add(chart);
        root
    }
}

fn create_bar_chart(label: &str, value: f64, color: &str) -> Document {
    const BAR_TOP: f32 = 27.5;
    const BAR_HEIGHT: i32 = 8;
    const BAR_ROUND: i32 = 5;
    let mut root = Document::new().set("width", CHART_WIDTH);
    let text = Text::new()
        .set("x", 0)
        .set("y", 20)
        .add(svg::node::Text::new(format!("{}", label)));
    let whole_rect = Rectangle::new()
        .set("x", 0)
        .set("y", BAR_TOP)
        .set("rx", BAR_ROUND)
        .set("ry", BAR_ROUND)
        .set("width", CHART_WIDTH)
        .set("height", BAR_HEIGHT)
        .set("fill", "#ddd")
        .set("class", "whole");
    let ratio_rect = Rectangle::new()
        .set("x", 0)
        .set("y", BAR_TOP)
        .set("rx", BAR_ROUND)
        .set("ry", BAR_ROUND)
        .set("width", format!("{}%", value))
        .set("height", BAR_HEIGHT)
        .set("fill", color)
        .set("class", "ratio");

    root = root.add(text).add(whole_rect).add(ratio_rect);

    root
}

fn create_header_pane(count: i64, x: i32, y: i32) -> Document {
    let mut root = Document::new().set("class", "star").set("x", x).set("y", y);

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

fn create_footer_pane(x: i32, y: i32) -> Document {
    let now = Utc::now();
    let current_date = now.format("%Y-%m-%d").to_string();

    let text_before = Text::new()
        .set("x", 0)
        .set("y", 20)
        .add(svg::node::Text::new("Generated by"));

    let link_text = Text::new()
        .set("x", 80)
        .set("y", 20)
        .add(svg::node::Text::new("github-stats-generator"));

    let link = Link::new()
        .set("href", "https://github.com/kengo-k/github-stats-generator")
        .add(link_text);

    let text_after = Text::new()
        .set("x", 210)
        .set("y", 20)
        .add(svg::node::Text::new(format!("at {}", current_date)));

    let root = Document::new()
        .set("class", "footer")
        .set("x", x)
        .set("y", y)
        .add(text_before)
        .add(link)
        .add(text_after);

    root
}

fn create_chart_title(title: &str, x: i32, y: i32) -> Text {
    let title = Text::new()
        .set("x", x)
        .set("y", y)
        .set("class", "title")
        .add(svg::node::Text::new(title));
    title
}

fn create_gradient(id: &str, from: &str, to: &str) -> LinearGradient {
    let stop_from = Stop::new()
        .set("offset", "0%")
        .set("style", format!("stop-color: {}", from));
    let stop_to = Stop::new()
        .set("offset", "100%")
        .set("style", format!("stop-color: {}", to));
    let root = LinearGradient::new()
        .set("id", id)
        .set("x1", "0%")
        .set("y1", "0%")
        .set("x2", "100%")
        .set("y2", "100%")
        .add(stop_from)
        .add(stop_to);
    root
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
