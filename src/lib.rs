use std::time::Duration;

use crate::repo::Repo;
use credentials::Credentials;
use main_service::MainService;
use reddit_service::RedditService;
use shuttle_secrets::SecretStore;
use tracing::info;
pub mod collectible;
pub mod credentials;
pub mod main_service;
pub mod reddit_service;
pub mod reddit_service_builder;
pub mod repo;
pub mod utils;

#[shuttle_service::main]
async fn init(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> Result<MainService, shuttle_service::Error> {
    let credentials = Credentials::new(secret_store)?;
    info!("Credentials loaded succesfully.");

    Ok(MainService {
        repo: Repo::new(include_str!("../items.json"))?,
        reddit_service: RedditService::new(credentials)
            .subreddit("onlyfans")
            .sleep_time(Duration::from_secs(5))
            .build()
            .await?,
    })
}
