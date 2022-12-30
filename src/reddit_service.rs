use crate::{
    collectible::{self, Collectible},
    credentials::Credentials,
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

    pub async fn reply(
        &mut self,
        comment_fullname: &str,
        collectibles: &[&crate::collectible2::Collectible],
    ) {
        let mut collectibles = collectibles.to_vec();
        collectibles.truncate(5);

        let body = Self::get_body2(&collectibles);

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

            let quality = match c.quality {
                Some(quality) => format!(", **Quality:** {}", quality),
                None => String::new(),
            };

            let second_line = format!("**Type:** {}{}\n\n", item_type, quality);
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

    fn get_body2(collectibles: &[&crate::collectible2::Collectible]) -> String {
        use crate::collectible2::*;
        use CollectibleType::*;
        use ItemType::*;
        use NonPickupType::*;

        let mut all = String::new();

        for collectible in collectibles {
            let name = &collectible.name;
            let desc = &collectible.description;
            let wiki_link = &collectible.wiki_link;
            let unlock = &collectible.unlock;
            let mut id_ = None;
            let mut collectible_type_;
            let mut recharge_time_ = None;
            let mut item_pool_ = None;
            let mut quality_ = None;
            let mut quote_ = None;

            match &collectible.collectible_type {
                NonPickup {
                    id,
                    quote,
                    non_pickup_type,
                } => {
                    id_ = Some(id);
                    quote_ = Some(quote);
                    match non_pickup_type {
                        Item { quality, item_type } => {
                            collectible_type_ = "Item".to_string();
                            quality_ = Some(quality);

                            match item_type {
                                Passive => collectible_type_.insert_str(0, "Passive "),
                                Active {
                                    recharge_time,
                                    item_pool,
                                } => {
                                    collectible_type_.insert_str(0, "Active ");
                                    recharge_time_ = Some(recharge_time);
                                    item_pool_ = item_pool.clone();
                                }
                            }
                        }
                        Trinket => collectible_type_ = "Trinket".to_string(),
                        Consumable => collectible_type_ = "Card/Pill/Rune".to_string(),
                    };
                }
                Pickup => collectible_type_ = "Pickup".to_string(),
            }

            let mut body = String::new();

            let id = match id_ {
                Some(id) => format!("[Id: {}]", id),
                None => String::new(),
            };

            let quote = match quote_ {
                Some(q) => format!(" - *\"{}\"*", q),
                None => String::new(),
            };

            let first_line = format!("{} [{}]({}){}\n\n", id, name, wiki_link, quote);
            body.push_str(&first_line);

            let quality = match quality_ {
                Some(quality) => format!(", **Quality:** {}", quality),
                None => String::new(),
            };

            let second_line = format!("**Type:** {}{}\n\n", collectible_type_, quality);
            body.push_str(&second_line);

            if let Some(time) = recharge_time_ {
                body.push_str(&format!("**Recharge time:** {}\n", time))
            };

            body.push_str("\n---\n\n");
            body.push_str(&Self::get_description_formatted(desc));
            body.push_str("\n\n---\n\n");

            if let Some(pool) = item_pool_.as_ref() {
                body.push_str(&format!("**Item pool:** {}\n\n", pool));
            }

            if let Some(unlock) = unlock.as_ref() {
                body.push_str(&format!("**Unlock:** {}\n\n", unlock));
            }

            all.push_str(&body);
            all.push_str("\n\n");
            all.push_str("&nbsp;\n\n");
        }

        all
    }
}
