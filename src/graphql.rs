use crate::generated::github_stats::git_hub_stats;
use crate::generated::github_stats::git_hub_stats::{
    GitHubStatsViewerRepositoriesNodesDefaultBranchRef,
    GitHubStatsViewerRepositoriesNodesDefaultBranchRefTarget,
};
use crate::generated::github_stats::GitHubStats;
use crate::graphql::git_hub_stats::ResponseData;
use crate::AppError;
use graphql_client::GraphQLQuery;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::env;

///
/// Definitions for mapping GraphQL custom types to Rust types
///
pub mod custom_scalars {
    pub type DateTime = chrono::DateTime<chrono::Utc>;
    pub type GitTimestamp = String;
}

///
/// A wrapper struct for deserializing the GraphQL response
///
#[derive(Deserialize, Serialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

///
/// A struct that normalizes the response from GraphQL for easier handling
///
#[derive(Serialize, Debug, Clone)]
pub struct RepositoryStat {
    pub id: String,
    pub name: String,
    pub is_private: bool,
    pub is_fork: bool,
    pub is_archived: bool,
    pub is_template: bool,
    pub disk_usage: i64,
    pub stargazer_count: i64,
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    pub topics: Vec<String>,
    pub languages: Vec<RepositoryLanguage>,
    pub total_commit_count: i64,
    pub period_commit_count: i64,
}

///
/// Data about the languages used in the repository
///
#[derive(Serialize, Debug, Clone)]
pub struct RepositoryLanguage {
    pub name: String,
    pub color: String,
    pub size: i64,
}

///
/// Generate a client to access the GitHub API
///
fn get_client() -> Result<RequestBuilder, AppError> {
    let client = Client::builder()
        .user_agent("MyApp/0.1")
        .build()
        .map_err(|_| AppError::GraphQLClientInitError);
    let token = env::var("GITHUB_TOKEN").expect("error: GITHUB_TOKEN is not set");
    client.map(|c| c.post("https://api.github.com/graphql").bearer_auth(token))
}

///
/// Get repository statistics using the GitHub GraphQL API
///
pub async fn get_github_stats(
    from: String,
    to: String,
) -> Result<GraphQLResponse<ResponseData>, AppError> {
    let client = get_client()?;
    let query = GitHubStats::build_query(git_hub_stats::Variables { from, to });

    let response = client
        .json(&query)
        .send()
        .await
        .map_err(|_| AppError::GraphQLRequestError)?;

    let body_text = response
        .text()
        .await
        .map_err(|_| AppError::GraphQLResponseError)?;

    let response: GraphQLResponse<ResponseData> =
        serde_json::from_str(&body_text).map_err(|_| AppError::JsonDeserializeError)?;

    Ok(response)
}

pub fn normalize(response: ResponseData) -> Vec<RepositoryStat> {
    let nodes = response.viewer.repositories.nodes.unwrap_or_else(Vec::new);
    let nodes: Vec<_> = nodes.into_iter().filter_map(|item| item).collect();
    let default_date: chrono::DateTime<chrono::Utc> = "9999-12-31T00:00:00Z".parse().unwrap();

    let mut result = Vec::new();
    for node in nodes {
        let topics: Vec<_> = node
            .repository_topics
            .edges
            .unwrap_or_else(Vec::new)
            .into_iter()
            .filter_map(|item| item)
            .map(|t| t.node)
            .filter_map(|t| t)
            .map(|t| t.topic.name)
            .collect();

        let languages: Vec<_> = node
            .languages
            .map(|n| n.edges)
            .unwrap_or_else(|| None)
            .unwrap_or_else(Vec::new)
            .into_iter()
            .filter_map(|item| item)
            .map(|item| {
                let lang = RepositoryLanguage {
                    name: item.node.name,
                    color: item.node.color.unwrap_or("red".to_string()),
                    size: item.size,
                };
                lang
            })
            .collect();

        let commit_count = get_commit_count(node.default_branch_ref).unwrap_or((0, 0));

        let item = RepositoryStat {
            id: node.id,
            name: node.name,
            is_private: node.is_private,
            is_fork: node.is_fork,
            is_archived: node.is_archived,
            is_template: node.is_template,
            disk_usage: node.disk_usage.unwrap_or(-1),
            stargazer_count: node.stargazer_count,
            pushed_at: node.pushed_at.unwrap_or(default_date),
            topics,
            languages,
            total_commit_count: commit_count.0,
            period_commit_count: commit_count.1,
        };
        result.push(item);
    }
    result
}

fn get_commit_count(
    branch_ref: Option<GitHubStatsViewerRepositoriesNodesDefaultBranchRef>,
) -> Option<(i64, i64)> {
    let branch_ref = branch_ref?;
    let target = branch_ref.target?;
    let result = match target {
        GitHubStatsViewerRepositoriesNodesDefaultBranchRefTarget::Commit(commit) => Some(commit),
        _ => None,
    };
    let result = result?;
    Some((
        result.commit_history_all.total_count,
        result.commit_history_period.total_count,
    ))
}
