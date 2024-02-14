use async_trait::async_trait;
use crate::entities;
use crate::utils::E;

pub mod speedb;

#[async_trait]
pub trait Database: Send + Sync {
    async fn insert_note(&self, _: entities::Note) -> Result<(), E>;
    async fn get_user_notes(&self, _: i64) -> Result<Vec<entities::Note>, E>;
    async fn search_notes(&self, _:i64, _: String, _: Vec<f64>) -> Result<Vec<entities::Note>, E>;
}
