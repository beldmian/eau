use crate::utils::E;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    #[serde(alias = "http_server")]
    pub http_server_config: HttpServerConfig,
    #[serde(alias = "bot")]
    pub bot_config: BotConfig,
    #[serde(alias = "auth")]
    pub auth_config: AuthConfig,
    #[serde(alias = "database")]
    pub database_config: DatabaseConfig,
    #[serde(alias = "ai")]
    pub ai_config: AIConfig,
}

#[derive(Clone, Deserialize)]
pub struct HttpServerConfig {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub token: String,
}

#[derive(Deserialize)]
pub enum DatabaseConfig {
    #[serde(alias = "speedb")]
    SpeeDB(SpeeDBConfig),
}

#[derive(Deserialize)]
pub struct SpeeDBConfig {
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
    #[serde(alias = "local_huggingface")]
    Local(LocalHFConfig),
}

#[derive(Deserialize)]
pub struct HFConfig {
    pub token: String,
    pub models_pipeline: Vec<String>,
}

#[derive(Deserialize)]
pub struct LocalHFConfig {
    pub model_name: String,
}

pub fn get_config(path: &str) -> Result<Config, E> {
    let f = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&f)?;
    Ok(config)
}
