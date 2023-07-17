use crate::{AppError, config};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use crate::generated::top_languages;
use crate::generated::top_languages::top_languages::ResponseData;
use crate::generated::top_languages::top_languages::Variables;

#[derive(Deserialize)]
struct GraphQLResponse {
    pub data: ResponseData,
}

#[derive(Debug)]
pub struct SvgData {
    pub name: String,
    pub size: i64,
    pub ratio: f64,
    pub color: String,
}

pub async fn get_github_summary() -> Result<Vec<SvgData>, AppError> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set");
    let client = Client::builder()
        .user_agent("MyApp/0.1")
        .build()
        .map_err(|_| AppError::GraphQLError)?;
    let query = top_languages::TopLanguages::build_query(Variables {});

    let response = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .json(&query)
        .send()
        .await
        .map_err(|_| AppError::GraphQLError)?;

    let body_text = response.text().await.map_err(|_| AppError::GraphQLError)?;
    let response: GraphQLResponse =
        serde_json::from_str(&body_text).map_err(|_| AppError::GraphQLError)?;

    let data = to_svg_data(&response.data)?;
    let data: Vec<SvgData> = data.into_iter().map(|(_, v)| v).collect();

    Ok(data)
}

fn to_svg_data(stats: &ResponseData) -> Result<HashMap<String, SvgData>, AppError> {
    let config = config::load();
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

            if config.ignore_languages.contains(name) {
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
