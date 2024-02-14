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

impl database::Database for SpeeDbDatabase {
    fn insert_note<'a,'async_trait>(
        &'a self,
        note: entities::Note,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<(), E>> + core::marker::Send+'async_trait>>
        where 'a: 'async_trait, Self: 'async_trait
    {
        async fn run(_self: &SpeeDbDatabase, note: entities::Note) -> Result<(), E> {
            _self.db.use_ns("eau").use_db("note").await?;
            _self.db.create::<Vec<Record>>("notes").content(note).await?;
            Ok(())
        }
        Box::pin(run(self, note))
    }
    fn get_user_notes<'a,'async_trait>(
        &'a self,
        owner_telegram_id: i64,
    ) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Vec<entities::Note>, E>> + core::marker::Send+'async_trait>>
        where 'a: 'async_trait,Self: 'async_trait
    {
        async fn run(_self: &SpeeDbDatabase, owner_telegram_id: i64) -> Result<Vec<entities::Note>, E> {
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
        search_text: String,
        search_embedding: Vec<f64>
    ) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Vec<entities::Note>, E>> + core::marker::Send+'async_trait>>
        where 'a: 'async_trait, Self: 'async_trait {
        async fn run(_self: &SpeeDbDatabase, owner_telegram_id: i64, search_text: String, search_embedding: Vec<f64>) -> Result<Vec<entities::Note>, E> {
            _self.db.use_ns("eau").use_db("note").await?;
            Ok(_self.db
                .query("SELECT *, search_score / max_score[0].max_score + (1 + sim) / 2 AS total_score FROM (SELECT *, (SELECT math::max(search::score(1)) AS max_score FROM notes WHERE owner_telegram_id == $id AND (text @1@ $search_text OR 1) GROUP ALL) AS max_score, vector::similarity::cosine(embedding, $search_embedding) AS sim, search::score(1) AS search_score FROM notes WHERE owner_telegram_id == $id AND (text @1@ $search_text OR 1)) ORDER BY total_score DESC")
                .bind(("id", owner_telegram_id))
                .bind(("search_embedding", search_embedding))
                .bind(("search_text", search_text))
                .await?.take(0)?)
        }
        Box::pin(run(self, owner_telegram_id, search_text, search_embedding))
    }
}

impl SpeeDbDatabase {
    pub async fn new(path: &str) -> Result<impl database::Database, E> {
        let db = Surreal::new::<SpeeDb>(path).await?;
        db.use_ns("eau").use_db("note").await?;
        db.query("DEFINE ANALYZER ascii TOKENIZERS class FILTERS ascii;").await?;
        db.query("DEFINE INDEX noteText ON TABLE notes COLUMNS text SEARCH ANALYZER ascii BM25 HIGHLIGHTS;").await?;
        Ok(Self {
            db: Arc::new(db),
        })
    }
}

