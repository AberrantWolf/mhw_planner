use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum ArmorType {
    Head,
    Chest,
    Gloves,
    Waist,
    Legs,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum ArmorRank {
    Low,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
struct Defense {
    base: i32,
    max: i32,
    augmented: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Resistances {
    fire: i32,
    water: i32,
    ice: i32,
    thunder: i32,
    dragon: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct SetInfo {
    id: i32,
    name: String,
    rank: ArmorRank,
    pieces: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ArmorAssets {
    image_male: String,
    image_female: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArmorCraftingInfo {
    materials: Vec<CraftingCost>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ArmorAttributes {
    required_gender: Option<Gender>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArmorInfo {
    id: i32,
    name: String,
    #[serde(rename = "type")]
    type_val: ArmorType,
    rank: ArmorRank,
    rarity: i32,
    defense: Defense,
    resistances: Resistances,
    slots: Vec<Slot>,
    skills: Vec<SkillRank>,
    set: SetInfo,
    assets: ArmorAssets,
    crafting: ArmorCraftingInfo,
    attributes: ArmorAttributes,
}
