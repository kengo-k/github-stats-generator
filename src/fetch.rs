use crate::generated::query::github_stats::{ResponseData, Variables};
use crate::generated::query::GithubStats;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct GraphQLResponse {
    pub data: ResponseData,
}

pub async fn get_github_summary() -> Result<ResponseData, Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set");
    let client = Client::builder().user_agent("MyApp/0.1").build()?;
    let query = GithubStats::build_query(Variables {});

    let response = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .json(&query)
        .send()
        .await?;

    let body_text = response.text().await?;
    let response: GraphQLResponse = serde_json::from_str(&body_text)?;

    Ok(response.data)
}
