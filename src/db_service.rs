use anyhow::Result;
use sqlx::{query_as, Executor, PgPool, Postgres};
use tracing::info;

use crate::collectible::Collectible;

pub struct DbService {
    pool: sqlx::PgPool,
}

impl DbService {
    pub async fn new(pool: sqlx::PgPool, db_data: Vec<Collectible>) -> Result<Self> {
        pool.execute(include_str!("../schema.sql")).await?;
        info!("Schema loaded into database succesfully.");

        for value in db_data {
            sqlx::query("INSERT INTO collectibles (kind, name, quote, quality, unlock, item_type, recharge_time, item_pool, description, wiki_link) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
                .bind(format!("{:?}", value.kind))
                .bind(value.name)
                .bind(value.quote)
                .bind(value.quality)
                .bind(value.unlock)
                .bind(value.item_type)
                .bind(value.recharge_time)
                .bind(value.item_pool)
                .bind(value.description)
                .bind(value.wiki_link)
                .execute(&pool).await?;
        }

        info!("Collectibles inserted into database succesfully.");

        Ok(Self { pool })
    }

    pub async fn from_url(url: &str) -> Result<Self> {
        Ok(Self {
            pool: PgPool::connect(url).await?,
        })
    }

    pub async fn get_collectibles(&self, names: &[&str]) -> Vec<Collectible> {
        let mut res: Vec<Collectible> = vec![];

        for name in names {
            let lowercase = name.to_lowercase();
            let as_str = lowercase.as_str();

            let mut collectibles = match as_str {
                "broken shovel" | "cancer" | "ace of spades" => sqlx::query_as!(
                    Collectible,
                    "SELECT * FROM collectibles WHERE LOWER(name) LIKE $1 LIMIT 10",
                    format!("%{}%", as_str)
                )
                .fetch_all(&self.pool)
                .await
                .unwrap_or_else(|_| vec![]),
                name => sqlx::query_as!(
                    Collectible,
                    "SELECT * FROM collectibles WHERE LOWER(name) = $1 LIMIT 10",
                    name
                )
                .fetch_all(&self.pool)
                .await
                .unwrap_or_else(|_| vec![]),
            };

            res.append(&mut collectibles);
        }

        res
    }
}
