mod color;
mod config;
mod generated;
mod graphql;
mod renderer;

use chrono::{Duration, Utc};
use log::{debug, info};
use renderer::Renderer;

#[derive(Debug)]
pub enum AppError {
    RequestError,
    ResponseError,
    GraphQLClientInitError,
    GraphQLRequestError,
    GraphQLResponseError,
    JsonDeserializeError,
    ConvertError,
    SvgOutputError,
}

///
/// Calculate the duration of the most recent week.
///
fn get_date_range() -> (String, String) {
    let to = Utc::now();
    let from = to - Duration::days(7);
    (from.to_rfc3339(), to.to_rfc3339())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    env_logger::init();

    let date_range = get_date_range();
    info!("date range: from={}, to={}", date_range.0, date_range.1);

    let github_stats = graphql::get_github_stats(date_range.0, date_range.1).await?;

    let github_stats_string =
        serde_json::to_string_pretty(&github_stats).map_err(|_| AppError::ConvertError)?;
    debug!("graphql response: {}", github_stats_string);

    let github_stats = graphql::normalize(github_stats.data);

    let github_stats_json =
        serde_json::to_string(&github_stats).map_err(|_| AppError::ConvertError)?;
    std::fs::write("github_stats.json", github_stats_json).map_err(|_| AppError::ConvertError)?;

    let language_colors = color::get_language_color_settings()
        .await
        .map_err(|_| AppError::ConvertError)?;

    let mut renderer = Renderer::new(github_stats, language_colors);

    let github_stats_svg = renderer.render();
    std::fs::write("github_stats.svg", github_stats_svg.to_string())
        .map_err(|_| AppError::ConvertError)?;

    Ok(())
}
