use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum ItemType {
    Passive,
    Active {
        recharge_time: String,
        item_pool: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum CollectibleType {
    NonPickup {
        id: u32,
        quote: String,
        non_pickup_type: NonPickupType,
    },
    Pickup,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum NonPickupType {
    Item { quality: u8, item_type: ItemType },
    Trinket,
    Consumable,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Collectible {
    pub name: String,
    pub description: String,
    pub collectible_type: CollectibleType,
    pub unlock: Option<String>,
    pub wiki_link: String,
}

impl PartialOrd for Collectible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Collectible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
