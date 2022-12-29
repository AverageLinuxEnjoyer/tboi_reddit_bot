use std::time::Duration;

use credentials::Credentials;
use db_service::DbService;
use main_service::MainService;
use reddit_service::RedditService;
use shuttle_secrets::SecretStore;
use tracing::info;

use crate::collectible::Collectible;

pub mod collectible;
pub mod credentials;
pub mod db_service;
pub mod main_service;
pub mod reddit_service;
pub mod reddit_service_builder;
pub mod utils;

#[shuttle_service::main]
async fn init(
    // #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> Result<MainService, shuttle_service::Error> {
    let db_url_dev = secret_store
        .get("DB_URL_DEV")
        .ok_or_else(|| shuttle_service::Error::Secret("DB_URL couldn't be read.".into()))?;

    let pool = sqlx::PgPool::connect(&db_url_dev)
        .await
        .map_err(|err| shuttle_service::Error::Database(err.to_string()))?;

    let credentials = Credentials::new(secret_store)?;
    info!("Credentials loaded succesfully.");

    let db_data: Vec<Collectible> = serde_json::from_str(include_str!("../items.json"))
        .map_err(|err| shuttle_service::Error::Custom(anyhow::Error::from(err)))?;

    Ok(MainService {
        db_service: DbService::new(pool, db_data).await?,
        reddit_service: RedditService::new(credentials)
            .subreddit("bindingofisaac")
            .sleep_time(Duration::from_secs(15))
            .build()
            .await?,
    })
}
