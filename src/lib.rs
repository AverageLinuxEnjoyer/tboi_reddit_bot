use std::time::Duration;

use credentials::Credentials;
use db_service::DbService;
use main_service::MainService;
use reddit_service::RedditService;
use shuttle_secrets::SecretStore;
use tracing::info;

pub mod credentials;
pub mod db_service;
pub mod main_service;
pub mod reddit_service;

#[shuttle_service::main]
async fn init(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> Result<MainService, shuttle_service::Error> {
    let credentials = Credentials::new(secret_store)?;

    info!("Credentials loaded succesfully.");

    Ok(MainService {
        db_service: DbService::new(pool),
        reddit_service: RedditService::new(credentials)
            .subreddit("bindingofisaac")
            .sleep_time(Duration::from_secs(15))
            .build()
            .await?,
    })
}
