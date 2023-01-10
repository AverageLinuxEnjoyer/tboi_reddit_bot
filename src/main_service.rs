use crate::{logfile, reddit_service::RedditService, repo::Repo, utils::extract_strings_between};
use anyhow::{Error, Result};
use futures::StreamExt;
use roux_stream::stream_comments;
use shuttle_service::error::CustomError;
use std::time::Duration;
use tokio_retry::strategy::ExponentialBackoff;
use tracing::info;

pub struct MainService {
    pub repo: Repo,
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
        let retry_strategy = ExponentialBackoff::from_millis(5).factor(100).take(3);

        let mut stream = stream_comments(
            &self.reddit_service.subreddit,
            self.reddit_service.sleep_time,
            retry_strategy,
            Some(Duration::from_secs(10)),
        )
        .0;

        info!("Main service started.");
        logfile::logfile("Main service started.");

        loop {
            while let Some(comment) = stream.next().await {
                let (comment_fullname, comment_author, comment_body) = match || -> Result<_> {
                    let comment = comment?;

                    let id = comment.id.ok_or_else(|| Error::msg("no id"))?;
                    let fullname = format!("t1_{}", id);

                    let author = comment.author.ok_or_else(|| Error::msg("no author"))?;

                    let body = comment.body.ok_or_else(|| Error::msg("no body"))?;

                    Ok((fullname, author, body))
                }() {
                    Ok(res) => res,
                    Err(_) => continue,
                };

                if comment_body.contains("{{logs}}") {
                    let _ = self.reddit_service.reply_logs(&comment_fullname).await;
                }

                let keywords = extract_strings_between(&comment_body);

                if keywords.is_empty() {
                    continue;
                }

                let collectibles = self.repo.get_collectibles(&keywords);

                if collectibles.is_empty() {
                    continue;
                }

                let response = self
                    .reddit_service
                    .reply(&comment_fullname, &collectibles)
                    .await;

                let msg = match response {
                    Ok(_) => format!(
                        "Replied to [{}, {}] with info about: {:?}",
                        comment_author,
                        comment_fullname,
                        collectibles
                            .into_iter()
                            .map(|c| &c.name)
                            .collect::<Vec<&String>>()
                    ),
                    Err(err) => format!("Replying to a comment didn't work, error: {}", err),
                };

                info!(msg);
                logfile::logfile(&msg);
            }
        }
    }
}
