use crate::generated::top_languages::top_languages::ResponseData;
use crate::generated::top_languages::top_languages::Variables;
use crate::generated::{list_repositories, top_languages};
use crate::{config, AppError};
use graphql_client::GraphQLQuery;
use reqwest::{Client, RequestBuilder};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

pub mod custom_scalars {
    pub type DateTime = chrono::DateTime<chrono::Utc>;
}

#[derive(Deserialize)]
struct GraphQLResponse {
    pub data: ResponseData,
}

#[derive(Debug)]
pub struct LanguageSummary {
    pub name: String,
    pub size: i64,
    pub ratio: f64,
    pub color: String,
}

fn get_client() -> Result<RequestBuilder, AppError> {
    let client = Client::builder()
        .user_agent("MyApp/0.1")
        .build()
        .map_err(|_| AppError::GraphQLClientInitError);
    let token = env::var("GITHUB_TOKEN").expect("error: GITHUB_TOKEN is not set");
    client.map(|c| c.post("https://api.github.com/graphql").bearer_auth(token))
}

pub async fn list_repositories() -> Result<(), AppError> {
    let client = get_client()?;
    let query = list_repositories::ListRepositories::build_query(
        list_repositories::list_repositories::Variables {},
    );

    let response = client
        .json(&query)
        .send()
        .await
        .map_err(|_| AppError::GraphQLRequestError)?;

    let body_text = response
        .text()
        .await
        .map_err(|_| AppError::GraphQLResponseError)?;
    Ok(())
}

pub async fn get_top_languages() -> Result<Vec<LanguageSummary>, AppError> {
    let client = get_client()?;
    let query = top_languages::TopLanguages::build_query(Variables {});

    let response = client
        .json(&query)
        .send()
        .await
        .map_err(|_| AppError::GraphQLRequestError)?;

    let body_text = response
        .text()
        .await
        .map_err(|_| AppError::GraphQLResponseError)?;
    let response: GraphQLResponse =
        serde_json::from_str(&body_text).map_err(|_| AppError::JsonDeserializeError)?;

    let data = to_svg_data(&response.data)?;
    let data: Vec<LanguageSummary> = data.into_iter().map(|(_, v)| v).collect();

    Ok(data)
}

fn to_svg_data(stats: &ResponseData) -> Result<HashMap<String, LanguageSummary>, AppError> {
    let config = config::load();
    let mut data: HashMap<String, LanguageSummary> = HashMap::new();

    let viewer = &stats.viewer;
    let repositories = viewer
        .repositories
        .edges
        .as_ref()
        .ok_or(AppError::ConvertError)?;

    for repo in repositories {
        let repo_node = repo
            .as_ref()
            .ok_or(AppError::ConvertError)?
            .node
            .as_ref()
            .ok_or(AppError::ConvertError)?;

        let repo_langs = repo_node
            .languages
            .as_ref()
            .ok_or(AppError::ConvertError)?
            .edges
            .as_ref()
            .ok_or(AppError::ConvertError)?;

        for repo_lang in repo_langs {
            let repo_lang = repo_lang.as_ref().ok_or(AppError::ConvertError)?;
            let size = repo_lang.size;
            let name = &repo_lang.node.name;

            if config.ignore_languages.contains(name) {
                continue;
            }

            let color = repo_lang
                .node
                .color
                .as_ref()
                .ok_or(AppError::ConvertError)?;

            let entry = data.entry(name.to_string()).or_insert(LanguageSummary {
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
