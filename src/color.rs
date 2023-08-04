use crate::AppError;
use reqwest::get;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct LanguageColor {
    pub color: Option<String>,
}

///
/// Retrieve color information for each programming language used in GitHub repositories.
///
pub async fn get_language_color_settings() -> Result<HashMap<String, String>, AppError> {
    let url = "https://raw.githubusercontent.com/github/linguist/master/lib/linguist/languages.yml";
    let resp = get(url)
        .await
        .map_err(|_| AppError::RequestError)?
        .text()
        .await
        .map_err(|_| AppError::ResponseError)?;

    let language_colors: HashMap<String, LanguageColor> =
        serde_yaml::from_str(&resp).map_err(|_| AppError::ConvertError)?;

    let mut result: HashMap<String, String> = HashMap::new();
    for lang in language_colors.keys() {
        let lang_color = language_colors.get(lang);
        if let Some(c) = lang_color {
            if let Some(color_string) = &c.color {
                result.insert(lang.to_string(), color_string.to_string());
            }
        }
    }
    Ok(result)
}

mod tests {

    #[tokio::test]
    async fn test_get_language_color_settings() {
        let colors = crate::color::get_language_color_settings().await.unwrap();
        assert_ne!(colors.len(), 0);

        assert!(colors.contains_key("Rust"));
        assert!(colors.contains_key("Go"));
        assert!(colors.contains_key("Elm"));

        let rust = colors.get("Rust");
        assert!(rust.is_some());
        let rust_color = rust.unwrap();
        assert!(rust_color.starts_with("#"));
    }
}
