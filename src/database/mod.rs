use async_trait::async_trait;
use crate::entities;
pub mod speedb;

#[async_trait]
pub trait Database: Send + Sync {
    async fn insert_note(&self, _: entities::Note) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_user_notes(&self, _: i64) -> Result<Vec<entities::Note>, Box<dyn std::error::Error>>;
}
