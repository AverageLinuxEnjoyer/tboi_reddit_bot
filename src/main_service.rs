use crate::{
    db_service::DbService, reddit_service::RedditService, repo::Repo,
    utils::extract_strings_between,
};
use anyhow::{Error, Result};
use futures::StreamExt;
use roux_stream::stream_comments;
use shuttle_service::error::CustomError;
use std::time::Duration;
use tokio_retry::strategy::ExponentialBackoff;
use tracing::info;

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
    async fn start(&mut self) -> Result<(), CustomError> {
        info!("Main service started.");

        let retry_strategy = ExponentialBackoff::from_millis(5).factor(100).take(3);

        let mut stream = stream_comments(
            &self.reddit_service.subreddit,
            self.reddit_service.sleep_time,
            retry_strategy,
            Some(Duration::from_secs(10)),
        )
        .0;

        let repo = Repo::new()?;

        while let Some(comment) = stream.next().await {
            let (fullname, body) = match || -> Result<_> {
                let comment = comment?;

                let id = comment.id.ok_or_else(|| Error::msg("no id"))?;
                let fullname = format!("t1_{}", id);

                let body = comment.body.ok_or_else(|| Error::msg("no body"))?;

                Ok((fullname, body))
            }() {
                Ok(res) => res,
                Err(_) => continue,
            };

            let keywords = extract_strings_between(&body);
            let keywords_as_refs = keywords.iter().map(|c| c.as_str()).collect::<Vec<_>>();

            if keywords.is_empty() {
                continue;
            }

            // let collectibles = self.db_service.get_collectibles(&keywords_as_refs).await;
            let mut collectibles = repo.get_collectibles(&keywords_as_refs);

            if collectibles.is_empty() {
                continue;
            }

            collectibles.truncate(5);

            self.reddit_service.reply(&fullname, &collectibles).await;
        }

        todo!();
    }
}
