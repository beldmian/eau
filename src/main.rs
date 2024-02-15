mod ai;
mod bot;
mod config;
mod database;
mod entities;
mod utils;

#[tokio::main]
async fn main() -> Result<(), utils::E> {
    let config = config::get_config("config.toml")?;
    let ai_api = match config.ai_config {
        config::AIConfig::HF(hf_config) => {
            ai::huggingface::HFApi::new(&hf_config.token, &hf_config.models_pipeline)
        }
    };
    let db = database::speedb::SpeeDbDatabase::new(&config.database_path).await?;
    let bot = bot::BotServer::new(Box::new(db), Box::new(ai_api), &config.telegram_token).await;
    bot.start().await
}
