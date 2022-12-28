use std::{thread::sleep, time::Duration};

use shuttle_secrets::SecretStore;
use shuttle_service::error::CustomError;
use tracing::info;

use crate::{credentials::Credentials, db_service::DbService, reddit_service::RedditService};

pub struct MainService {
    pub db_service: DbService,
    pub reddit_service: RedditService,
}

#[shuttle_service::async_trait]
impl shuttle_service::Service for MainService {
    async fn bind(
        mut self: Box<Self>,
        _addr: std::net::SocketAddr,
    ) -> Result<(), shuttle_service::error::Error> {
        self.start().await?;

        Ok(())
    }
}

impl MainService {
    async fn start(&self) -> Result<(), CustomError> {
        info!("Main service started.");
        loop {
            sleep(Duration::from_secs(10));
        }
    }
}
