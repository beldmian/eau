use crate::ai::AIApi;
use crate::auth::AuthProvider;
use crate::config::BotConfig;
use crate::database::Database;
use crate::utils::E;
use futures::StreamExt;
use std::sync::Arc;
use telegram_bot::{Api, UpdateKind};

mod handlers;

#[derive(Clone)]
pub struct BotServer {
    bot: Api,
    db: Arc<dyn Database>,
    auth_provider: Arc<dyn AuthProvider>,
    ai_api: Arc<Box<dyn AIApi>>,
}

impl BotServer {
    pub async fn new(
        db: Arc<dyn Database>,
        ai_api: Arc<Box<dyn AIApi>>,
        auth_provider: Arc<dyn AuthProvider>,
        config: &BotConfig,
    ) -> BotServer {
        let bot = Api::new(config.token.clone());
        Self {
            bot,
            db,
            ai_api,
            auth_provider,
        }
    }
    pub async fn start(&self) -> Result<(), E> {
        let mut stream = self.bot.stream();
        while let Some(update) = stream.next().await {
            let update = update?;
            if let UpdateKind::Message(message) = update.kind {
                let _self = self.clone();
                let _message = message.clone();
                tokio::spawn(async move {
                    _self.message_handler(_message).await.unwrap();
                });
            }
        }
        Ok(())
    }
}
