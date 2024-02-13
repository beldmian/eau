use futures::StreamExt;
use telegram_bot::{Api, UpdateKind};
use crate::database::{self, Database};

mod handlers;

pub struct BotServer {
    bot: Api, 
    db: Box<dyn Database>,
}

impl BotServer {
    pub async fn new(db: Box<dyn database::Database>, token: &str) -> BotServer {
        let bot = Api::new(token);
        Self {
            bot,
            db,
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
