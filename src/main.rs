mod generated;
mod graphql;
mod renderer;
mod config;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum AppError {
    JsonCreateFailure,
    JsonExtractValueFailure,
    JsonPublishFailure,
    GraphQLError,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = config::load();
    let mut data = graphql::get_github_summary()
        .await
        .map_err(|_| AppError::GraphQLError)?;

    data.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());
    data.truncate(config.languages_count);

    let svg_data = renderer::write(&data)?;
    let file = File::create("image.svg");
    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            return Err(AppError::JsonPublishFailure);
        }
    };
    let _ = file.write_all(svg_data.as_bytes());

    Ok(())
}
