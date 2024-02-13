use crate::entities;
use crate::ai::get_embedding_retrying;
use crate::bot;
use telegram_bot::*;

impl bot::BotServer {
    pub async fn message_handler(&self, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
        if let MessageKind::Text { ref data, .. } = msg.kind {
            match data.as_ref() {
                "/list" => {
                    let notes = self.db.get_user_notes(msg.chat.id().into()).await?;
                    let message = notes.iter().enumerate().map(|(i, note)| {
                        format!("{}. {}\n", i+1, note.text)
                    }).fold("".to_string(), |acc, x| format!("{}{}", acc, x));
                    self.bot.send(msg.text_reply(format!("Notes: \n{}", message))).await?;
                    msg
                },
                _ => {
                    self.bot.send(msg.text_reply("New note added")).await?;
                    let embedding = get_embedding_retrying(&"hello".to_string()).await?;
                    self.db.insert_note(entities::Note{
                        text: data.clone(),
                        owner_telegram_id: msg.chat.id().into(),
                        embedding,
                    }).await?;
                    self.bot.send(msg.text_reply("New note indexed")).await?;
                    msg
                },
            };
        }

        Ok(())
    }
}
