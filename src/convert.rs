use std::collections::HashMap;
use crate::AppError;
use crate::generated::query::github_stats::ResponseData;

#[derive(Debug)]
pub struct SvgData {
    pub name: String,
    pub size: i64,
    pub ratio: f64,
    pub color: String,
}

pub fn to_svg_data(stats: &ResponseData) -> Result<HashMap<String, SvgData>, AppError> {
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
