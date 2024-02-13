use crate::entities;
use crate::ai::get_embedding_retrying;
use crate::bot;
use telegram_bot::*;

impl bot::BotServer {
    pub fn format_notes_list(&self, notes: Vec<entities::Note>) -> String {
        notes.iter().enumerate().map(|(i, note)| {
            format!("{}. {}\n", i+1, note.text)
        }).fold("".to_string(), |acc, x| format!("{}{}", acc, x))
    }
    pub async fn list_notes_handler(&self, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
        let notes = self.db.get_user_notes(msg.chat.id().into()).await?;
        self.bot.send(msg.text_reply(format!("Notes: \n{}", self.format_notes_list(notes)))).await?;
        Ok(())
    }
    pub async fn add_note_handler(&self, msg: Message, text: String) -> Result<(), Box<dyn std::error::Error>> {
        self.bot.send(msg.text_reply("New note added")).await?;
        let embedding = get_embedding_retrying(&text).await?;
        self.db.insert_note(entities::Note{
            text,
            owner_telegram_id: msg.chat.id().into(),
            embedding,
        }).await?;
        self.bot.send(msg.text_reply("New note indexed")).await?;
        Ok(())
    }
    pub async fn search_notes_query(&self, msg: Message, query: String) -> Result<(), Box<dyn std::error::Error>> {
        self.bot.send(msg.text_reply("Searching...")).await?;
        let query_embedding = get_embedding_retrying(&query).await?;
        let search_result = self.db.search_notes(msg.chat.id().into(), query_embedding).await?;
        self.bot.send(msg.text_reply(format!("Notes: \n{}", self.format_notes_list(search_result)))).await?;
        Ok(())
    }
    pub async fn message_handler(&self, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
        if let MessageKind::Text { ref data, .. } = msg.kind {
            if data.starts_with("/list") {
                self.list_notes_handler(msg.clone()).await?
            } else if let Some(query) = data.strip_prefix("/search") {
                self.search_notes_query(msg.clone(), String::from(query)).await?
            } else {
                self.add_note_handler(msg.clone(), data.clone()).await?
            }
        }
        Ok(())
    }
}
