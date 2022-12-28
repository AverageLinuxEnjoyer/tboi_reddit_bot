use std::time::Duration;

use crate::credentials::{self, Credentials};
use anyhow::Result;
use roux::{Me, Reddit, Subreddit};

pub struct RedditService {
    client: Me,
    subreddit: Subreddit,
    sleep_time: Duration,
}

#[allow(clippy::new_ret_no_self)]
impl RedditService {
    pub fn new(credentials: Credentials) -> RedditServiceBuilder {
        RedditServiceBuilder {
            credentials,
            subreddit: None,
            sleep_time: None,
        }
    }
}

pub struct RedditServiceBuilder {
    credentials: Credentials,
    subreddit: Option<Subreddit>,
    sleep_time: Option<Duration>,
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
