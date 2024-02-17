use crate::entities;
use crate::utils::E;
use async_trait::async_trait;

pub mod speedb;

#[async_trait]
pub trait Database: NoteRepository {}
impl<T> Database for T where T: NoteRepository {}

#[async_trait]
pub trait NoteRepository: Send + Sync {
    async fn insert_note(&self, _: entities::Note) -> Result<(), E>;
    async fn get_user_notes(&self, _: i64) -> Result<Vec<entities::Note>, E>;
    async fn search_notes(&self, _: i64, _: String, _: Vec<f64>) -> Result<Vec<entities::Note>, E>;
}
