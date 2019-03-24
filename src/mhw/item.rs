use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Item {
    id: i32,
    name: String,
    description: String,
    rarity: i32,
    carry_limit: i32,
    value: i32,
}
