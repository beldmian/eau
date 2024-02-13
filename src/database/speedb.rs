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
        where Self: Sized, 'a: 'async_trait, Self: 'async_trait
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
        where Self: Sized,'a: 'async_trait,Self: 'async_trait
    {
        async fn run(_self: &SpeeDbDatabase, owner_telegram_id: i64) -> Result<Vec<entities::Note>, Box<dyn std::error::Error>> {
            _self.db.use_ns("eau").use_db("note").await.unwrap();
            Ok(_self.db
                .query("SELECT * FROM notes WHERE owner_telegram_id == $id")
                .bind(("id", owner_telegram_id))
                .await?.take(0)?)
        }
        Box::pin(run(self, owner_telegram_id))
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

