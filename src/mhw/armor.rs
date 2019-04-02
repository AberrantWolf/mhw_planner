use super::common::{
    fonts::*, rarity::*, CraftingCost, GuiDetails, MhwEvent, MhwWindowContents, SkillRank, Slot,
};
use crate::widgets::*;
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

impl Display for ArmorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ArmorRank {
    Low,
    High,
}

impl Display for ArmorRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Defense {
    pub base: i32,
    pub max: i32,
    pub augmented: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resistances {
    pub fire: i32,
    pub water: i32,
    pub ice: i32,
    pub thunder: i32,
    pub dragon: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetInfo {
    pub id: i32,
    pub name: String,
    pub rank: ArmorRank,
    pub pieces: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArmorAssets {
    pub image_male: Option<String>,
    pub image_female: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArmorCraftingInfo {
    pub materials: Vec<CraftingCost>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Gender {
    Male,
    Female,
}

impl Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArmorAttributes {
    pub required_gender: Option<Gender>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArmorInfo {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_val: ArmorType,
    pub rank: ArmorRank,
    pub rarity: u32,
    pub defense: Defense,
    pub resistances: Resistances,
    pub slots: Vec<Slot>,
    pub skills: Vec<SkillRank>,
    pub armor_set: Option<SetInfo>,
    pub assets: ArmorAssets,
    pub crafting: ArmorCraftingInfo,
    pub attributes: ArmorAttributes,

    // internal details
    #[serde(skip)]
    pub resistances_cache: SimpleTableDataModel,
    #[serde(skip)]
    pub skills_cache: SimpleTableDataModel,
    #[serde(skip)]
    pub crafting_cache: SimpleTableDataModel,
    #[serde(skip)]
    pub other_cache: SimpleTableDataModel,
}

impl ArmorInfo {
    pub fn resistances_data(&mut self) -> &TableDataModel {
        macro_rules! try_add_resistance_row {
            ($name:expr, $elem:ident) => {
                if self.resistances.$elem != 0 {
                    self.resistances_cache.push($name.to_owned());
                    self.resistances_cache
                        .push(self.resistances.$elem.to_string());
                }
            };
        }
        if self.resistances_cache.is_empty() {
            self.resistances_cache.set_columns(2);
            // see if we need to add anything
            try_add_resistance_row!("Fire", fire);
            try_add_resistance_row!("Water", water);
            try_add_resistance_row!("Ice", ice);
            try_add_resistance_row!("Thunder", thunder);
            try_add_resistance_row!("Dragon", dragon);
        }

        &self.resistances_cache
    }

    pub fn skills_data(&mut self) -> &TableDataModel {
        if self.skills_cache.is_empty() {
            self.skills_cache.set_columns(2);
            // see if we need to add anything
            let skills = &self.skills;
            let iter = skills.iter();
            let skills_cache = &mut self.skills_cache;
            for skill in iter {
                skills_cache.push(skill.skill_name.clone());
                skills_cache.push(skill.level.to_string());
            }
        }

        &self.skills_cache
    }

    pub fn crafting_data(&mut self) -> &TableDataModel {
        if self.crafting_cache.is_empty() {
            self.crafting_cache.set_columns(2);
            let mats = &self.crafting.materials;
            for cost in mats {
                self.crafting_cache.push(cost.item.name.clone());
                self.crafting_cache.push(cost.quantity.to_string());
            }
        }
        &self.crafting_cache
    }

    pub fn other_data(&mut self) -> &TableDataModel {
        if self.other_cache.is_empty() {
            self.other_cache.set_columns(2);
            if let Some(gender) = &self.attributes.required_gender {
                self.other_cache.push("Required Gender".to_owned());
                self.other_cache.push(gender.to_string());
            }
        }

        &self.other_cache
    }
}

impl MhwWindowContents for ArmorInfo {
    fn build_window<'a>(
        &mut self,
        ui: &Ui<'a>,
        _details: &mut GuiDetails,
        _event_queue: &mut VecDeque<MhwEvent>, // will need this when linking between objects
    ) {
        //=======================================
        // Name/ID
        ui.with_font(FONT_IDX_HEADER, || {
            let imstring = ImString::new(self.name.clone());
            ui.text_colored(rarity_color(self.rarity), &imstring);
        });
        ui.with_font(FONT_IDX_MINI, || {
            let id_string = format!("id: [{}]", self.id);
            ui.text(id_string.as_str());
            ui.same_line(0.0);
            ui.text(format!("{} Rank {}", self.rank, self.type_val));
        });

        //=======================================
        // Core Stats
        ui.columns(3, im_str!("armor_stats"), true);
        // Defense
        ui.with_font(FONT_IDX_WINDOW_TITLE, || {
            let text = im_str!("Defense");
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
            ui.text(text);
        });
        ui.with_font(FONT_IDX_NORMAL, || {
            let text = String::from(match &self.armor_set {
                Some(ref set) => set.name.as_str(),
                None => "<none>",
            });

            let imgstr = ImString::from(text);
            ui.text(&imgstr);
        });

        //=======================================
        // Lists section
        ui.columns(2, im_str!("armor_attribs"), true);
        ui.separator();
        draw_table(ui, "Resistances", self.resistances_data());

        ui.next_column();
        draw_table(ui, "Skills", self.skills_data());

        ui.next_column();
        draw_table(ui, "Crafting", self.crafting_data());

        ui.next_column();
        draw_table(ui, "Other Attribs", self.other_data());
    }
}
