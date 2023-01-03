use anyhow::Result;
use distance::damerau_levenshtein;
use std::collections::{HashMap, HashSet};
use tracing::info;

use crate::collectible::{Collectible, CollectibleType, ItemType, NonPickupType};
pub struct Repo {
    pub collectibles: Vec<Collectible>,
    pub redirects: HashMap<&'static str, Vec<&'static str>>,
}

impl Repo {
    pub fn new(data_file: &str) -> Result<Self> {
        let mut collectibles: Vec<Collectible> = serde_json::from_str(data_file)?;
        collectibles.sort();

        let mut redirects = HashMap::new();
        redirects.insert("Cancer", vec!["Cancer (Trinket)", "Cancer (Item)"]);
        redirects.insert(
            "Odd Mushroom",
            vec!["Odd Mushroom (Thin)", "Odd Mushroom (Large)"],
        );

        Ok(Self {
            collectibles,
            redirects,
        })
    }

    pub fn get_collectibles(&self, names: &[String]) -> Vec<&Collectible> {
        let mut res = Vec::new();
        let mut redirects = vec![];

        for name in names {
            let name = name.to_lowercase();
            let maybe_id = name.parse::<u32>();

            match maybe_id {
                Ok(certainly_id) => {
                    for c in &self.collectibles {
                        // test for id
                        if let CollectibleType::NonPickup {
                            id,
                            non_pickup_type: NonPickupType::Item { .. },
                            ..
                        } = &c.collectible_type
                        {
                            if certainly_id == *id {
                                res.push(c);
                                break;
                            }
                        }
                    }
                }
                Err(_) => {
                    for c in &self.collectibles {
                        if damerau_levenshtein(&c.name.to_lowercase(), &name) <= 1 {
                            res.push(c);
                            break; // we only add the first similar name found
                        }
                    }

                    for redirect in &self.redirects {
                        if damerau_levenshtein(&redirect.0.to_lowercase(), &name) <= 1 {
                            redirects.push(*redirect.0);
                            for r in redirect.1 {
                                for c in &self.collectibles {
                                    if &c.name == r {
                                        res.push(c);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // eliminate duplicates from res
        let mut res_set = HashSet::new();
        res.retain(|x| res_set.insert(*x));

        self.handle_special_redirects(&redirects);
        res
    }

    fn handle_special_redirects(&self, redirects: &[&str]) {
        for redirect in redirects {
            match *redirect {
                "tmtrainer" => (),
                "lol" => (),
                _ => (),
            }
        }
    }
}
