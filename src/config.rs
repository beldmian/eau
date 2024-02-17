use crate::utils::E;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    #[serde(alias = "bot")]
    pub bot_config: BotConfig,
    #[serde(alias = "auth")]
    pub auth_config: AuthConfig,
    #[serde(alias = "database")]
    pub database_config: DatabaseConfig,
    #[serde(alias = "ai")]
    pub ai_config: AIConfig,
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub token: String,
}

#[derive(Deserialize)]
pub enum DatabaseConfig {
    #[serde(alias = "local")]
    Local(DatabaseLocalConfig),
}

#[derive(Deserialize)]
pub struct DatabaseLocalConfig {
    pub path: String,
}

#[derive(Deserialize)]
pub enum AuthConfig {
    #[serde(alias = "jwt")]
    JWT(JWTAuthConfig),
}

#[derive(Deserialize)]
pub struct JWTAuthConfig {
    pub secret: String,
}

#[derive(Deserialize)]
pub enum AIConfig {
    #[serde(alias = "huggingface")]
    HF(HFConfig),
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
