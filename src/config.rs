use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub languages_count: usize,
    pub ignore_repositories: Vec<String>,
    pub ignore_languages: Vec<String>,
    pub language_mapping: HashMap<String, String>,
    pub rename_language: HashMap<String, String>,
}

pub fn load() -> Config {
    let config_string = fs::read_to_string("config.toml").expect("error: fail to load config.toml");
    let config_obj: Config = toml::from_str(&config_string).expect("error: fail to create Config");
    config_obj
}
