use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Kind {
    Item,
    Trinket,
    Pickup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collectible {
    pub kind: Kind,

    pub name: String,
    pub quote: Option<String>,
    pub quality: Option<u8>,

    pub unlock: Option<String>,
    pub item_type: Option<String>,
    pub recharge_time: Option<String>,
    pub item_pool: Option<String>,

    pub description: String,
}

impl Display for Collectible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)?;
        writeln!(f, " {{")?;
        writeln!(f, "\tname: \"{}\",", self.name)?;
        writeln!(f, "\tquote: {:?},", self.quote)?;
        writeln!(f, "\tquality: {:?},", self.quality)?;
        writeln!(f, "\tunlock: {:?},", self.unlock)?;
        writeln!(f, "\titem_type: {:?},", self.item_type)?;
        writeln!(f, "\trecharge_time: {:?},", self.recharge_time)?;
        writeln!(f, "\titem_pool: {:?},", self.item_pool)?;
        writeln!(f, "\tdescription:")?;
        write!(f, "{}", "=".repeat(80))?;
        writeln!(f, "{}", self.description)?;
        writeln!(f, "{}", "=".repeat(80))?;
        write!(f, "}}")?;

        Ok(())
    }
}
