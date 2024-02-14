use serde::Deserialize;
use std::fs;
use crate::utils::E;

#[derive(Deserialize)]
pub struct Config {
    pub telegram_token: String,
    pub database_path: String,
    #[serde(alias="ai")]
    pub ai_config: HFConfig,
}

#[derive(Deserialize)]
pub struct HFConfig {
    pub token: String,
    pub models_pipeline: Vec<String>,
}

pub fn get_config(path: &str) -> Result<Config, E> {
    let f = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&f)?;
    Ok(config)
}
