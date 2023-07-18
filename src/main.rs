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
    let mut data = graphql::get_top_languages().await?;

    data.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());
    data.truncate(config.languages_count);

    let svg_data = renderer::write(&data)?;
    let file = File::create("image.svg");
    let mut file = file.map_err(|_| AppError::SvgOutputError)?;
    let _ = file.write_all(svg_data.as_bytes());

    Ok(())
}
