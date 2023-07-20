mod config;
mod generated;
mod graphql;
mod renderer;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum AppError {
    GraphQLClientInitError,
    GraphQLRequestError,
    GraphQLResponseError,
    JsonDeserializeError,
    ConvertError,
    SvgOutputError,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = config::load();

    let mut top_languages = graphql::get_top_languages().await?;
    top_languages.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());

    let top_languages_json = serde_json::to_string(&top_languages).map_err(|_| AppError::ConvertError)?;
    std::fs::write("top_lang.json", top_languages_json).map_err(|_| AppError::ConvertError)?;

    top_languages.truncate(config.languages_count);

    let all_repos = graphql::list_repositories().await?;

    let svg_data = renderer::write(&top_languages, &all_repos)?;
    let file = File::create("image.svg");
    let mut file = file.map_err(|_| AppError::SvgOutputError)?;
    let _ = file.write_all(svg_data.as_bytes());

    Ok(())
}
