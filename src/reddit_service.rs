use crate::{
    collectible::Collectible,
    credentials::{self, Credentials},
    reddit_service_builder::RedditServiceBuilder,
};
use anyhow::Result;
use roux::{Me, Reddit, Subreddit};
use std::time::Duration;
use tracing::info;

pub struct RedditService {
    pub client: Me,
    pub subreddit: Subreddit,
    pub sleep_time: Duration,
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

    pub async fn reply(&mut self, comment_fullname: &str, collectibles: &[Collectible]) {
        let body = String::new();

        let a = format!("{:?}", collectibles);
        info!(a);
    }
}
