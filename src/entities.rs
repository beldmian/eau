use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub text: String,
    pub embedding: Vec<f64>,
    pub owner_telegram_id: i64,
}
