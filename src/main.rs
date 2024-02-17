use std::sync::Arc;
mod ai;
mod auth;
mod bot;
mod config;
mod database;
mod entities;
mod http;
mod utils;

#[tokio::main]
async fn main() -> Result<(), utils::E> {
    let config = config::get_config("config.toml")?;
    let ai_api = Arc::new(match config.ai_config {
        config::AIConfig::HF(hf_config) => ai::huggingface::HFApi::new(&hf_config),
    });
    let db = Arc::new(match config.database_config {
        config::DatabaseConfig::SpeeDB(speedb_config) => {
            database::speedb::SpeeDbDatabase::new(&speedb_config).await?
        }
    });
    let auth_provider = Arc::new(match config.auth_config {
        config::AuthConfig::JWT(jwt_config) => auth::jwt::JWTAuthProvider::new(&jwt_config)?,
    });
    let bot = bot::BotServer::new(
        db.clone(),
        ai_api.clone(),
        auth_provider.clone(),
        &config.bot_config,
    )
    .await;
    let http_server = http::HttpServer::new(
        db.clone(),
        ai_api.clone(),
        auth_provider.clone(),
        &config.http_server_config,
    )
    .await;
    let app = tokio::spawn(async move {
        let (r1, r2) = tokio::join!(bot.start(), http_server.start());
        match r1 {
            Ok(_) => println!("bot shut down"),
            Err(e) => println!("bot err {}", e),
        }
        match r2 {
            Ok(_) => println!("http_server shut down"),
            Err(e) => println!("http_server err {}", e),
        }
    });
    tokio::select! {
        _ = app => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    Ok(())
}
