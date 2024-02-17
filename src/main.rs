mod ai;
mod auth;
mod bot;
mod config;
mod database;
mod entities;
mod utils;

#[tokio::main]
async fn main() -> Result<(), utils::E> {
    let config = config::get_config("config.toml")?;
    let ai_api = match config.ai_config {
        config::AIConfig::HF(hf_config) => ai::huggingface::HFApi::new(&hf_config),
    };
    let db = match config.database_config {
        config::DatabaseConfig::SpeeDB(speedb_config) => {
            database::speedb::SpeeDbDatabase::new(&speedb_config).await?
        }
    };
    let auth_provider = match config.auth_config {
        config::AuthConfig::JWT(jwt_config) => auth::jwt::JWTAuthProvider::new(&jwt_config)?,
    };
    let bot = bot::BotServer::new(
        Box::new(db),
        Box::new(ai_api),
        Box::new(auth_provider),
        &config.bot_config,
    )
    .await;
    bot.start().await
}
