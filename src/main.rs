mod convert;
mod fetch;
mod generated;
mod publish;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum AppError {
    JsonCreateFailure,
    JsonExtractValueFailure,
    JsonPublishFailure,
    GraphQLError,
}

trait JsonValueExtension {
    fn to_int(&self) -> Result<i64, AppError>;
    fn to_float(&self) -> Result<f64, AppError>;
}

impl JsonValueExtension for serde_json::Value {
    fn to_int(&self) -> Result<i64, AppError> {
        self.as_i64().ok_or(AppError::JsonExtractValueFailure)
    }
    fn to_float(&self) -> Result<f64, AppError> {
        self.as_f64().ok_or(AppError::JsonExtractValueFailure)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> Self {
        AppError::JsonCreateFailure
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let github_summary = fetch::get_github_summary()
        .await
        .map_err(|_| AppError::GraphQLError)?;

    let data = convert::to_svg_data(&github_summary)?;
    let mut data: Vec<convert::SvgData> = data.into_iter().map(|(_, v)| v).collect();
    data.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());
    data.truncate(10);

    let svg_data = publish::write()?;
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
