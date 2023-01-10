pub mod collectible;
pub mod credentials;
pub mod logfile;
pub mod main_service;
pub mod reddit_service;
pub mod reddit_service_builder;
pub mod repo;
pub mod utils;

use crate::repo::Repo;
use credentials::Credentials;
use main_service::MainService;
use reddit_service::RedditService;
use shuttle_secrets::SecretStore;
use std::time::Duration;
use tracing::info;

#[shuttle_service::main]
async fn init(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> Result<MainService, shuttle_service::Error> {
    let credentials = Credentials::new(secret_store)?;

    info!("Credentials loaded succesfully.");
    logfile::logfile("Credentials loaded succesfully.");

    let content = std::fs::read_to_string("items.json")?;

    Ok(MainService {
        repo: Repo::new(&content)?,
        reddit_service: RedditService::new(credentials)
            .subreddit("bindingofisaac")
            .sleep_time(Duration::from_secs(5))
            .build()
            .await?,
    })
}
