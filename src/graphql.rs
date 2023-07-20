use crate::generated::list_repositories::list_repositories;
use crate::generated::list_repositories::list_repositories::ListRepositoriesViewerRepositoriesNodes;
use crate::generated::list_repositories::ListRepositories;
use crate::generated::top_languages::top_languages;
use crate::generated::top_languages::TopLanguages;
use crate::{config, AppError};
use graphql_client::GraphQLQuery;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

pub mod custom_scalars {
    pub type DateTime = chrono::DateTime<chrono::Utc>;
}

#[derive(Deserialize)]
struct GraphQLResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct LanguageSummary {
    pub name: String,
    pub size: i64,
    pub ratio: f64,
    pub color: String,
}

#[derive(Debug, Serialize)]
pub struct RepositoryData {
    pub name: String,
    pub is_private: bool,
    pub is_fork: bool,
    pub is_archived: bool,
    pub is_template: bool,
    pub disk_usage: i64,
    pub stargazer_count: i64,
    pub pushed_at: String,
    pub repository_topics: Vec<String>,
}

fn get_client() -> Result<RequestBuilder, AppError> {
    let client = Client::builder()
        .user_agent("MyApp/0.1")
        .build()
        .map_err(|_| AppError::GraphQLClientInitError);
    let token = env::var("GITHUB_TOKEN").expect("error: GITHUB_TOKEN is not set");
    client.map(|c| c.post("https://api.github.com/graphql").bearer_auth(token))
}

pub async fn list_repositories() -> Result<Vec<RepositoryData>, AppError>
{
    let client = get_client()?;
    let query = ListRepositories::build_query(list_repositories::Variables {});

    let response = client
        .json(&query)
        .send()
        .await
        .map_err(|_| AppError::GraphQLRequestError)?;

    let body_text = response
        .text()
        .await
        .map_err(|_| AppError::GraphQLResponseError)?;

    let response: GraphQLResponse<list_repositories::ResponseData> =
        serde_json::from_str(&body_text).map_err(|_| AppError::JsonDeserializeError)?;

    let nodes = response
        .data
        .viewer
        .repositories
        .nodes
        .ok_or(AppError::ConvertError)?;

    let values = vec![Some(1), None];
    let values2: Vec<_> = values.into_iter().filter_map(|aaa|aaa).collect();

    let mut result: Vec<RepositoryData> = Vec::new();
    for node in nodes {
        let node = node.ok_or(AppError::ConvertError)?;
        let x = node.repository_topics.edges.unwrap_or_else(Vec::new);
        let y: Vec<_> = x.into_iter().filter_map(|bbb| bbb).collect();
        let z: Vec<_> = y.into_iter().map(|x| x.node).filter_map(|ccc| ccc).collect();
        let zzz: Vec<_> = z.into_iter().map(|zzz| zzz.topic.name).collect();

        let repo = RepositoryData {
            name: node.name,
            is_private: node.is_private,
            is_fork: node.is_fork,
            is_archived: node.is_archived,
            is_template: node.is_template,
            disk_usage: node.disk_usage.unwrap_or(-1),
            stargazer_count: node.stargazer_count,
            pushed_at: node.pushed_at.map(|d| d.to_string()).unwrap_or("".to_string()),
            repository_topics: zzz
        };
        result.push(repo);
    }
    Ok(result)
}

pub async fn get_top_languages() -> Result<Vec<LanguageSummary>, AppError> {
    let client = get_client()?;
    let query = TopLanguages::build_query(top_languages::Variables {});

    let response = client
        .json(&query)
        .send()
        .await
        .map_err(|_| AppError::GraphQLRequestError)?;

    let body_text = response
        .text()
        .await
        .map_err(|_| AppError::GraphQLResponseError)?;
    let response: GraphQLResponse<top_languages::ResponseData> =
        serde_json::from_str(&body_text).map_err(|_| AppError::JsonDeserializeError)?;

    let data = to_svg_data(&response.data)?;
    let data: Vec<LanguageSummary> = data.into_iter().map(|(_, v)| v).collect();

    Ok(data)
}

fn to_svg_data(
    stats: &top_languages::ResponseData,
) -> Result<HashMap<String, LanguageSummary>, AppError> {
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

    let mut sum = 0;
    for (_, value) in &data {
        sum += value.size;
    }
    for lang in data.keys().cloned().collect::<Vec<_>>() {
        if let Some(d) = data.get_mut(&lang) {
            d.ratio = d.size as f64 / sum as f64 * 100.0;
        }
    }

    Ok(data)
}
