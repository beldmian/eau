use crate::bot;
use crate::entities;
use crate::utils::E;
use telegram_bot::*;

impl<'a> bot::BotServer {
    pub fn format_notes_list(&'a self, notes: Vec<entities::Note>) -> String {
        notes
            .iter()
            .enumerate()
            .map(|(i, note)| format!("{}. {}\n", i + 1, note.text))
            .fold("".to_string(), |acc, x| format!("{}{}", acc, x))
    }
    pub async fn list_notes_handler(&'a self, msg: Message) -> Result<(), E> {
        let notes = self.db.get_user_notes(msg.chat.id().into()).await?;
        self.bot
            .send(msg.text_reply(format!("Notes: \n{}", self.format_notes_list(notes))))
            .await?;
        Ok(())
    }
    pub async fn add_note_handler(&'a self, msg: Message, text: String) -> Result<(), E> {
        self.bot.send(msg.text_reply("New note added")).await?;
        let embedding = self.ai_api.get_embedding(&text).await?;
        self.db
            .insert_note(entities::Note {
                text,
                owner_telegram_id: msg.chat.id().into(),
                embedding,
            })
            .await?;
        self.bot.send(msg.text_reply("New note indexed")).await?;
        Ok(())
    }
    pub async fn search_notes_query(&'a self, msg: Message, query: String) -> Result<(), E> {
        self.bot.send(msg.text_reply("Searching...")).await?;
        let query_embedding = self.ai_api.get_embedding(&query).await?;
        let search_result = self
            .db
            .search_notes(msg.chat.id().into(), query, query_embedding)
            .await?;
        self.bot
            .send(msg.text_reply(format!(
                "Notes: \n{}",
                self.format_notes_list(search_result)
            )))
            .await?;
        Ok(())
    }
    pub async fn generate_token(&'a self, msg: Message) -> Result<(), E> {
        self.bot
            .send(
                msg.text_reply(format!(
                    "Your token:\n\n`{}`",
                    self.auth_provider
                        .create_secret(crate::auth::IdentificationPayload {
                            id: msg.chat.id().into()
                        })?
                ))
                .parse_mode(ParseMode::Markdown),
            )
            .await?;
        Ok(())
    }
    pub async fn message_handler(&'a self, msg: Message) -> Result<(), E> {
        if let MessageKind::Text { ref data, .. } = msg.kind {
            if data.starts_with("/list") {
                self.list_notes_handler(msg.clone()).await?
            } else if data.starts_with("/token") {
                self.generate_token(msg.clone()).await?
            } else if let Some(query) = data.strip_prefix("/search ") {
                self.search_notes_query(msg.clone(), String::from(query))
                    .await?
            } else {
                self.add_note_handler(msg.clone(), data.clone()).await?
            }
        }
        Ok(())
    }
}
