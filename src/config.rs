use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ignore_languages: Vec<String>,
    pub languages_count: usize
}

pub fn load() -> Config {
    let config_string = fs::read_to_string("config.toml").expect("error: fail to load config.toml");
    let config_obj: Config = toml::from_str(&config_string).expect("error: fail to create Config");
    config_obj
}
