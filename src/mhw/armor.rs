use super::common::*;
use imgui::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ArmorType {
    Head,
    Chest,
    Gloves,
    Waist,
    Legs,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ArmorRank {
    Low,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Defense {
    base: i32,
    max: i32,
    augmented: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resistances {
    fire: i32,
    water: i32,
    ice: i32,
    thunder: i32,
    dragon: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetInfo {
    id: i32,
    name: String,
    rank: ArmorRank,
    pieces: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArmorAssets {
    image_male: String,
    image_female: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArmorCraftingInfo {
    materials: Vec<CraftingCost>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArmorAttributes {
    required_gender: Option<Gender>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
    armor_set: SetInfo,
    assets: ArmorAssets,
    crafting: ArmorCraftingInfo,
    attributes: ArmorAttributes,
}

impl MhwWindowContents for ArmorInfo {
    fn build_window<'a>(
        &mut self,
        ui: &Ui<'a>,
        details: &mut GuiDetails,
        event_queue: &mut VecDeque<MhwEvent>,
    ) {
        // TODO: Lay this out properly -- labels on the LEFT, and using the bigger font
        let imstring = ImString::new(self.name.clone());
        ui.label_text(im_str!("Name"), &imstring);
    }
}
