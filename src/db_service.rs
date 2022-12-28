use anyhow::Result;
use sqlx::{Executor, PgPool};
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
            sqlx::query("INSERT INTO collectibles (kind, name, quote, quality, unlock, item_type, recharge_time, item_pool) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
                .bind(format!("{:?}", value.kind))
                .bind(value.name)
                .bind(value.quote)
                .bind(value.quality)
                .bind(value.unlock)
                .bind(value.item_type)
                .bind(value.recharge_time)
                .bind(value.item_pool)
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

    pub async fn get_collectible(name: Vec<String>) -> Vec<Collectible> {
        vec![]
    }
}
