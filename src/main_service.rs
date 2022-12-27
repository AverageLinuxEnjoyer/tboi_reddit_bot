use std::{thread::sleep, time::Duration};

use shuttle_secrets::SecretStore;
use shuttle_service::error::CustomError;

use crate::db_service::DbService;

pub struct MainService {
    pub db_service: DbService,
    pub secret_store: SecretStore,
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
    async fn start(&self) -> Result<(), shuttle_service::error::CustomError> {
        let (_rec,): (String,) = sqlx::query_as("SELECT 'Hello world'")
            .fetch_one(&self.db_service.pool)
            .await
            .map_err(CustomError::new)?;

        loop {
            sleep(Duration::from_secs(10));
        }

        Ok(())
    }
}
