use super::common::{
    fonts::*, rarity::*, CraftingCost, GuiDetails, MhwEvent, MhwWindowContents, SkillRank, Slot,
};
use imgui::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt::{self, Debug, Display};

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

impl fmt::Display for ArmorRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
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
    image_male: Option<String>,
    image_female: Option<String>,
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
    rarity: u32,
    defense: Defense,
    resistances: Resistances,
    slots: Vec<Slot>,
    skills: Vec<SkillRank>,
    armor_set: Option<SetInfo>,
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
        //=======================================
        // Name/ID info
        ui.with_font(FONT_IDX_HEADER, || {
            let imstring = ImString::new(self.name.clone());
            ui.text_colored(rarity_color(self.rarity), &imstring);
        });
        ui.with_font(FONT_IDX_MINI, || {
            let id_string = format!("id: [{}]", self.id);
            ui.text(id_string.as_str());
            ui.same_line(0.0);
            ui.text(format!("{} Rank", self.rank));
        });

        //=======================================
        // Stats info
        ui.columns(3, im_str!("armor_stats"), true);
        // Defense
        ui.with_font(FONT_IDX_WINDOW_TITLE, || {
            let text = im_str!("Defense");
            // let width = ui.get_column_width(-1); // current column
            // let text_size = ui.calc_text_size(text, false, -1.0);
            // let remaining = width - text_size.x;
            // if remaining > 0.0 {
            //     let cursor = ui.get_cursor_pos();
            //     ui.set_cursor_pos((cursor.0 + remaining / 2.0, cursor.1));
            // }
            ui.text(text);
        });
        ui.with_font(FONT_IDX_NORMAL, || {
            ui.text("Base:");
            ui.same_line(0.0);
            ui.text(self.defense.base.to_string());

            ui.text("Max:");
            ui.same_line(0.0);
            ui.text(self.defense.max.to_string());

            ui.text("Augmented:");
            ui.same_line(0.0);
            ui.text(self.defense.augmented.to_string());
        });

        // Slots
        ui.next_column();
        ui.with_font(FONT_IDX_WINDOW_TITLE, || {
            let text = im_str!("Slot Info");
            ui.text(text);
        });
        for idx in 0..3 {
            match self.slots.get(idx) {
                Some(val) => match val.rank {
                    1 => {
                        ui.text("[1]");
                        ui.same_line(0.0);
                    }
                    2 => {
                        ui.text("[2]");
                        ui.same_line(0.0);
                    }
                    3 => {
                        ui.text("[3]");
                        ui.same_line(0.0);
                    }
                    _ => {
                        ui.text("[E]");
                        ui.same_line(0.0);
                    }
                },
                None => {
                    ui.text("[-]");
                    ui.same_line(0.0);
                }
            }
        }
        ui.new_line();

        // Set Info
        ui.next_column();
        ui.with_font(FONT_IDX_WINDOW_TITLE, || {
            let text = im_str!("Armor Set");
            // let width = ui.get_column_width(-1); // current column
            // let text_size = ui.calc_text_size(text, false, -1.0);
            // let remaining = width - text_size.x;
            // if remaining > 0.0 {
            //     let cursor = ui.get_cursor_pos();
            //     ui.set_cursor_pos((cursor.0 + remaining / 2.0, cursor.1));
            // }
            ui.text(text);
        });
        ui.with_font(FONT_IDX_NORMAL, || {
            let text = String::from(match &self.armor_set {
                Some(ref set) => set.name.as_str(),
                None => "<none>",
            });

            let imgstr = ImString::from(text);
            // let width = ui.get_column_width(-1); // current column
            // let text_size = ui.calc_text_size(&imgstr, false, -1.0);
            // let remaining = width - text_size.x;
            // if remaining > 0.0 {
            //     let cursor = ui.get_cursor_pos();
            //     ui.set_cursor_pos((cursor.0 + remaining / 2.0, cursor.1));
            // }
            ui.text(&imgstr);
        });

        ui.columns(2, im_str!("armor_skills"), true);
        ui.separator();
        ui.text("next");
    }
}
