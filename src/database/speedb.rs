use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::{Surreal, sql::Thing, engine::local::{SpeeDb, Db}};
use std::sync::Arc;

use crate::database;
use crate::entities;
use crate::utils::E;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub struct SpeeDbDatabase {
    db: Arc<Surreal<Db>>
}

#[async_trait]
impl database::Database for SpeeDbDatabase {
    async fn insert_note(
        &self,
        note: entities::Note,
    ) -> Result<(), E> {
        self.db.use_ns("eau").use_db("note").await?;
        self.db.create::<Vec<Record>>("notes").content(note).await?;
        Ok(())
    }
    async fn get_user_notes(
        &self,
        owner_telegram_id: i64,
    ) ->  Result<Vec<entities::Note>, E> {
        self.db.use_ns("eau").use_db("note").await?;
        Ok(self.db
            .query("SELECT * FROM notes WHERE owner_telegram_id == $id")
            .bind(("id", owner_telegram_id))
            .await?.take(0)?)
    }
    async fn search_notes(
        &self,
        owner_telegram_id: i64,
        search_text: String,
        search_embedding: Vec<f64>
    ) -> Result<Vec<entities::Note>, E> {
        self.db.use_ns("eau").use_db("note").await?;
        Ok(self.db
            .query(String::from_utf8_lossy(include_bytes!("queries/search.sql")).as_ref())
            .bind(("id", owner_telegram_id))
            .bind(("search_embedding", search_embedding))
            .bind(("search_text", search_text))
            .await?.take(0)?)
    }
}

impl SpeeDbDatabase {
    pub async fn new(path: &str) -> Result<impl database::Database, E> {
        let db = Surreal::new::<SpeeDb>(path).await?;
        db.use_ns("eau").use_db("note").await?;
        db.query(String::from_utf8_lossy(include_bytes!("queries/up.sql")).as_ref()).await?;
        Ok(Self {
            db: Arc::new(db),
        })
    }
}

