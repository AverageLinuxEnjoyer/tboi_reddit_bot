use crate::{
    collectible::Collectible, credentials::Credentials,
    reddit_service_builder::RedditServiceBuilder,
};
use roux::{Me, Subreddit};
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
        let mut collectibles = collectibles.to_vec();
        collectibles.truncate(5);

        let body = Self::get_body(&collectibles);

        let _ = self.client.comment(&body, comment_fullname).await;

        let msg = format!(
            "Replied to {} with info about: {}",
            comment_fullname,
            collectibles
                .into_iter()
                .map(|c| format!("[{}], ", c.name))
                .collect::<String>()
        );

        info!(msg);
    }

    fn get_body(collectibles: &[Collectible]) -> String {
        let mut all = String::new();

        for c in collectibles {
            let mut body = String::new();

            let quote = match c.quote.as_ref() {
                Some(q) => format!(" - *\"{}\"*", q),
                None => String::new(),
            };
            let first_line = format!("[{}]({}){}\n\n", c.name, c.wiki_link, quote);
            body.push_str(&first_line);

            let item_type = match c.item_type.as_ref() {
                Some(t) => format!("{} Item", t),
                None => c.kind.clone(),
            };
            let second_line = format!("**Type:** {}\n\n", item_type);
            body.push_str(&second_line);

            if let Some(time) = c.recharge_time.as_ref() {
                body.push_str(&format!("**Recharge time:** {}\n", time))
            };

            body.push_str("\n---\n\n");
            body.push_str(&Self::get_description_formatted(&c.description));
            body.push_str("\n\n---\n\n");

            if let Some(pool) = c.item_pool.as_ref() {
                body.push_str(&format!("**Item pool:** {}\n\n", pool));
            }

            if let Some(unlock) = c.unlock.as_ref() {
                body.push_str(&format!("**Unlock:** {}\n\n", unlock));
            }

            all.push_str(&body);
            all.push_str("\n\n");
            all.push_str("&nbsp;\n\n");
        }

        all.push_str(Self::get_footer());

        all
    }

    fn get_description_formatted(description_raw: &str) -> String {
        description_raw.replace('\n', "\n\n> ").trim().to_string()
    }

    fn get_footer() -> &'static str {
        "^(I am a bot and this action was performed automatically, for more info check my profile description. Data fetched from )^[platinumgod.](https://platinumgod.co.uk/)"
    }
}
