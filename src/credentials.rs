use anyhow::Error;
use anyhow::Result;
use shuttle_secrets::SecretStore;
use sqlx::FromRow;

#[derive(Debug)]
pub struct Credentials {
    pub user_agent: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
}

impl Credentials {
    pub fn new(secret_store: SecretStore) -> Result<Self> {
        Ok(Self {
            user_agent: secret_store
                .get("USER_AGENT")
                .ok_or_else(|| Error::msg("No user agent found."))?,
            client_id: secret_store
                .get("CLIENT_ID")
                .ok_or_else(|| Error::msg("No client id found."))?,
            client_secret: secret_store
                .get("CLIENT_SECRET")
                .ok_or_else(|| Error::msg("No client secret found."))?,
            username: secret_store
                .get("USERNAME")
                .ok_or_else(|| Error::msg("No username found."))?,
            password: secret_store
                .get("PASSWORD")
                .ok_or_else(|| Error::msg("No password found."))?,
        })
    }
}
