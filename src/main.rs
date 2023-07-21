mod config;
mod generated;
mod graphql;
mod renderer;

use chrono::{Duration, Utc};
use renderer::Renderer;

#[derive(Debug)]
pub enum AppError {
    GraphQLClientInitError,
    GraphQLRequestError,
    GraphQLResponseError,
    JsonDeserializeError,
    ConvertError,
    SvgOutputError,
}

fn get_date_range() -> (String, String) {
    let to = Utc::now();
    let from = to - Duration::days(7);
    (from.to_rfc3339(), to.to_rfc3339())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = config::load();

    let date_range = get_date_range();
    let github_stats = graphql::get_github_stats(date_range.0, date_range.1).await?;
    let github_stats = graphql::normalize(github_stats);
    let github_stats = github_stats
        .into_iter()
        .filter_map(|item| {
            if config.ignore_repositories.contains(&item.name) {
                None
            } else {
                Some(item)
            }
        })
        .collect();

    let github_stats_json =
        serde_json::to_string(&github_stats).map_err(|_| AppError::ConvertError)?;
    std::fs::write("github_stats.json", github_stats_json).map_err(|_| AppError::ConvertError)?;

    let renderer = Renderer::new(github_stats);
    let github_stats_svg = renderer.render();
    std::fs::write("github_stats.svg", github_stats_svg.to_string())
        .map_err(|_| AppError::ConvertError)?;

    Ok(())
}
