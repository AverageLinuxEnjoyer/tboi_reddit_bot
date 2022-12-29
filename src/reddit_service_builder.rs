use std::time::Duration;

use crate::{credentials::Credentials, reddit_service::RedditService};
use anyhow::Result;
use roux::{Reddit, Subreddit};

pub struct RedditServiceBuilder {
    pub credentials: Credentials,
    pub subreddit: Option<Subreddit>,
    pub sleep_time: Option<Duration>,
}

impl RedditServiceBuilder {
    pub async fn build(self) -> Result<RedditService> {
        Ok(RedditService {
            client: Reddit::new(
                &self.credentials.user_agent,
                &self.credentials.client_id,
                &self.credentials.client_secret,
            )
            .username(&self.credentials.username)
            .password(&self.credentials.password)
            .login()
            .await?,
            subreddit: self
                .subreddit
                .unwrap_or_else(|| Subreddit::new("bindingofisaac")),
            sleep_time: self.sleep_time.unwrap_or_else(|| Duration::from_secs(10)),
        })
    }

    pub fn subreddit(mut self, subreddit: &str) -> Self {
        self.subreddit = Some(Subreddit::new(subreddit));

        self
    }

    pub fn sleep_time(mut self, sleep_time: Duration) -> Self {
        self.sleep_time = Some(sleep_time);

        self
    }
}
