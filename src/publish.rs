use svg::Document;
use svg::node::element::Text;
use crate::AppError;

fn create_bar_chart(language: &str) -> Document {
    let mut document = Document::new().set("viewBox", (0, 0, 100, 100));
    let text = Text::new()
        .set("x", 0)
        .set("y", 0)
        .add(svg::node::Text::new(format!("{}", language)))
        ;
    document = document.add(text);
    document
}

pub fn write() -> Result<String, AppError> {
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sxd_document::{dom, Package, parser};
    use sxd_xpath::{Context, Error, evaluate_xpath, Factory, Value, XPath};

    fn xpath(path: &str) -> XPath {
        let factory = Factory::new();
        let xpath = factory.build(path).unwrap().unwrap();
        xpath
    }

    struct DocumentWrapper {
        package: Package,
        ns: Option<(String,String)>
    }

    impl DocumentWrapper {

        pub fn new(xml: &str) -> Self {
            let package = parser::parse(xml).unwrap();
            Self {
                package,
                ns: None
            }
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
            ns: Option<(String, String)>
        }

        impl Test {
            pub fn new(xpath: &str, source: &str, expected: &str) -> Self {
                Self {
                    xpath: xpath.to_string(),
                    source: source.to_string(),
                    expected: expected.to_string(),
                    ns: None
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
            Test::new("/ns:svg/@viewBox", r#"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg"></svg>"#, "0 0 100 100").set_namespace("ns", "http://www.w3.org/2000/svg"),
        ];

        for t in tests {
            let mut document = DocumentWrapper::new(t.source.as_str());
            if let Some(ns) = &t.ns {
                document = document.set_namespace(&ns.0, &ns. 1);
            }
            let value = document.string(t.xpath.as_str());
            assert_eq!(value, t.expected.as_str());
        }

    }

    #[test]
    fn test_create_bar_chart() {
        // let expected = r"<svg></svg>";
        // let expected = DocumentWrapper::new(expected);
        let actual = create_bar_chart("rust").to_string();
        //assert_eq!(actual, "yyyy");
        let actual = DocumentWrapper::new(actual.as_str());
        let viewBox = actual.string("/svg/@viewBox");
        //assert_eq!(viewBox, "xxxx");
        // println!("actual: {}", actual);
        // let actual = DocumentWrapper::new(actual.as_str());
        // let document = &actual.get();
        // //let value = evaluate_xpath(document, "/*/a + /*/b").unwrap();
        // //println!("HELLO");
        // //println!("{:?}", value.number());
        // let v = eval(document, "/*/a + /*/b");
        // assert_eq!(v.string(), "xxx");
    }
}