use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimal: u8,
    pub description: String,
    pub logo: String,
    pub creator: String,
}

pub const TOKEN: Item<TokenInfo> = Item::new("TokenInfo");
