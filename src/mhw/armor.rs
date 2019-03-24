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

impl MhwGui for ArmorInfo {
    fn layout<'a>(&mut self, ui: &Ui<'a>, event_queue: &mut VecDeque<MhwEvent>) {
        // TODO: Calculate remaining size
        let draw_cursor_pos = ui.get_cursor_pos();
        let window = ui
            .window(im_str!("Weapon Info"))
            .position(draw_cursor_pos, ImGuiCond::Always)
            .size((640f32, 480f32), ImGuiCond::Always);
        // TODO: Hide resize and collapse indicators (no resize!)

        window.build(|| {
            // TODO: fill this out... :P
        });
    }
}
