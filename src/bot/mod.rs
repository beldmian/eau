use crate::ai::AIApi;
use crate::database::{self, Database};
use crate::utils::E;
use futures::StreamExt;
use std::sync::Arc;
use telegram_bot::{Api, UpdateKind};

mod handlers;

#[derive(Clone)]
pub struct BotServer {
    bot: Api,
    db: Arc<Box<dyn Database>>,
    hf_api: Arc<Box<dyn AIApi>>,
}

impl<'a> BotServer {
    pub async fn new(
        db: Box<dyn database::Database>,
        hf_api: Box<dyn AIApi>,
        token: &str,
    ) -> BotServer {
        let bot = Api::new(token);
        Self {
            bot,
            db: Arc::new(db),
            hf_api: Arc::new(hf_api),
        }
    }
    pub async fn start(&'a self) -> Result<(), E> {
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
