use crate::collectible::*;
use crate::{credentials::Credentials, reddit_service_builder::RedditServiceBuilder};
use anyhow::{Error, Result};
use reqwest::Response;
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
        collectibles: &[&crate::collectible::Collectible],
    ) -> Result<Response> {
        // only allow a maximum of 5 items per reply
        //? maybe extract that magic number into a variable?
        let mut collectibles = collectibles.to_vec();
        collectibles.truncate(5);

        let body = Self::get_body(&collectibles);

        self.client
            .comment(&body, comment_fullname)
            .await
            .map_err(|err| Error::msg(err.to_string()))
    }

    //? is a function even needed here?
    fn get_description_formatted(description_raw: &str) -> String {
        description_raw.replace('\n', "\n\n> ").trim().to_string()
    }

    //? make this a const variable instead?
    //? or perhaps keep it a function, but let it return a String
    //? to allow the possibility of a dynamicly chosen footer?
    fn get_footer() -> &'static str {
        "^(I am a bot and this action was performed automatically, for more info check my profile description. Data fetched from )^[platinumgod.](https://platinumgod.co.uk/)"
    }

    // TODO: Refactor
    fn get_body(collectibles: &[&Collectible]) -> String {
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

        all.push_str(Self::get_footer());

        all
    }
}
