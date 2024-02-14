mod bot;
mod database;
mod entities;
mod ai;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config("config.toml")?;
    let hf_api = ai::HFApi::new(&config.ai_config.token, &config.ai_config.models_pipeline);
    let db = database::speedb::SpeeDbDatabase::new(&config.database_path).await?;
    let bot = bot::BotServer::new(Box::new(db), hf_api, &config.telegram_token).await;
    bot.start().await
}
