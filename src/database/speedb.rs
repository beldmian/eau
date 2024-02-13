use serde::Deserialize;
use surrealdb::{Surreal, sql::Thing, engine::local::{SpeeDb, Db}};
use std::sync::Arc;

use crate::database;
use crate::entities;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub struct SpeeDbDatabase {
    db: Arc<Surreal<Db>>
}

impl database::Database for SpeeDbDatabase {
    fn insert_note<'a,'async_trait>(
        &'a self,
        note: entities::Note,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + core::marker::Send+'async_trait>>
        where 'a: 'async_trait, Self: 'async_trait
    {
        async fn run(_self: &SpeeDbDatabase, note: entities::Note) -> Result<(), Box<dyn std::error::Error>> {
            _self.db.use_ns("eau").use_db("note").await?;
            _self.db.create::<Vec<Record>>("notes").content(note).await?;
            Ok(())
        }
        Box::pin(run(self, note))
    }
    fn get_user_notes<'a,'async_trait>(
        &'a self,
        owner_telegram_id: i64,
    ) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Vec<entities::Note>, Box<dyn std::error::Error>>> + core::marker::Send+'async_trait>>
        where 'a: 'async_trait,Self: 'async_trait
    {
        async fn run(_self: &SpeeDbDatabase, owner_telegram_id: i64) -> Result<Vec<entities::Note>, Box<dyn std::error::Error>> {
            _self.db.use_ns("eau").use_db("note").await?;
            Ok(_self.db
                .query("SELECT * FROM notes WHERE owner_telegram_id == $id")
                .bind(("id", owner_telegram_id))
                .await?.take(0)?)
        }
        Box::pin(run(self, owner_telegram_id))
    }
    fn search_notes<'a,'async_trait>(
        &'a self,
        owner_telegram_id: i64,
        search_embedding: Vec<f64>
    ) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Vec<entities::Note>, Box<dyn std::error::Error>>> + core::marker::Send+'async_trait>>
        where 'a: 'async_trait, Self: 'async_trait {
        async fn run(_self: &SpeeDbDatabase, owner_telegram_id: i64, search_embedding: Vec<f64>) -> Result<Vec<entities::Note>, Box<dyn std::error::Error>> {
            _self.db.use_ns("eau").use_db("note").await?;
            Ok(_self.db
                .query("SELECT *, vector::similarity::cosine(embedding, $search_embedding) AS similarity FROM notes WHERE owner_telegram_id == $id ORDER BY similarity DESC")
                .bind(("id", owner_telegram_id))
                .bind(("search_embedding", search_embedding))
                .await?.take(0)?)
        }
        Box::pin(run(self, owner_telegram_id, search_embedding))
    }
}

impl SpeeDbDatabase {
    pub async fn new(path: &str) -> Result<impl database::Database, Box<dyn std::error::Error>> {
        let db = Surreal::new::<SpeeDb>(path).await?;
        Ok(Self {
            db: Arc::new(db),
        })
    }
}

