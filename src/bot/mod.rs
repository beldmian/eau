use futures::StreamExt;
use telegram_bot::{Api, UpdateKind};
use crate::database::{self, Database};
use crate::ai::HFApi;

mod handlers;

pub struct BotServer {
    bot: Api, 
    db: Box<dyn Database>,
    hf_api: HFApi,
}

impl BotServer {
    pub async fn new(db: Box<dyn database::Database>, hf_api: HFApi, token: &str) -> BotServer {
        let bot = Api::new(token);
        Self {
            bot,
            db,
            hf_api,
        }
    }
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = self.bot.stream();
        while let Some(update) = stream.next().await {
            let update = update?;
            if let UpdateKind::Message(message) = update.kind {
                self.message_handler(message).await?;
            }
        }
        Ok(())
    }
}
