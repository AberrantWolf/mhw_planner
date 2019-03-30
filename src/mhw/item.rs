use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub rarity: i32,
    pub carry_limit: i32,
    pub value: i32,
}
