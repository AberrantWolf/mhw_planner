use super::common::{
    fonts::*, rarity::*, CraftingCost, Element, GuiDetails, MhwEvent, MhwWindowContents, SkillRank,
    Slot,
};
use crate::widgets::*;
use imgui::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt::{self, Debug, Display};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum WeaponType {
    GreatSword,
    LongSword,
    SwordAndShield,
    DualBlades,
    Hammer,
    HuntingHorn,
    Lance,
    Gunlance,
    SwitchAxe,
    ChargeBlade,
    InsectGlaive,
    LightBowgun,
    HeavyBowgun,
    Bow,
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    pub display: i32,
    pub raw: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeaponCraftingInfo {
    pub craftable: bool,
    pub previous: Option<i32>,
    pub branches: Vec<i32>,
    pub crafting_materials: Vec<CraftingCost>,
    pub upgrade_materials: Vec<CraftingCost>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeaponElement {
    #[serde(rename = "type")]
    pub elememt: Element,
    pub damage: i32,
    pub hidden: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeaponAssets {
    pub icon: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
// to get the percentage value of each segment, divide by 400
// For 100px view, the pixel width = ([sharpness] * 100) / 400
// using integer-only maths.
pub struct WeaponSharpness {
    pub red: i32,
    pub orange: i32,
    pub yellow: i32,
    pub green: i32,
    pub blue: i32,
    pub white: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmmoCapacities {
    pub normal: Vec<i32>,
    pub flaming: Vec<i32>,
    pub piercing: Vec<i32>,
    pub water: Vec<i32>,
    pub spread: Vec<i32>,
    pub freeze: Vec<i32>,
    pub sticky: Vec<i32>,
    pub thunder: Vec<i32>,
    pub cluster: Vec<i32>,
    pub dragon: Vec<i32>,
    pub recover: Vec<i32>,
    pub slicing: Vec<i32>,
    pub poison: Vec<i32>,
    pub wyvern: Vec<i32>,
    pub paralysis: Vec<i32>,
    pub demon: Vec<i32>,
    pub sleep: Vec<i32>,
    pub armor: Vec<i32>,
    pub exhaust: Vec<i32>,
    pub tranq: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BoostType {
    Sever,
    Speed,
    Element,
    Health,
    Stamina,
    Blunt,
}

impl Display for BoostType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Coating {
    #[serde(rename = "close range")]
    CloseRange, // TODO: comes through as "close range", so... need something for that
    Paralysis,
    Poison,
    Sleep,
    Blast,
    Power,
}

impl Display for Coating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DamageType {
    Blunt,
    Piercing,
    Slashing,
    Sever,
    Projectile,
}

impl Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Deviation {
    None,
    Low,
    Average,
    High,
}

impl Display for Deviation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Elderseal {
    Low,
    Average,
    High,
}

impl Display for Elderseal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PhialType {
    Impact,
    Element,
    Power,
    PowerElement,
    Dragon(i32),  // TODO: These come in as, e.g., "dragon 300"... so I'll
    Exhaust(i32), // need to make some kind of custom deserializer for them.
    Para(i32),    // At the moment, deserialization of these will fail.
    Poison(i32),
}

impl Display for PhialType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ShellingType {
    Normal(i32), // TODO: these come in as, e.g., "Normal Lv2", so will need some
    Long(i32),   // customg deserialization here. :( Currently, all gunlances fail.
    Wide(i32),
}

impl Display for ShellingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SpecialAmmoType {
    #[serde(rename = "wyvernblast")]
    Blast,
    #[serde(rename = "wyvernheart")]
    Heart,
    #[serde(rename = "wyvernsnipe")]
    Snipe,
}

impl Display for SpecialAmmoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeaponAttributes {
    pub ammo_capacities: Option<AmmoCapacities>, //For "light-bowgun" and "heavy-bowgun" weapons only
    pub affinity: Option<i32>,                   //The affinity of the weapon
    pub boost_type: Option<BoostType>,           //For "insect-glaive" weapons only
    pub coatings: Option<Vec<Coating>>,          //For "bow" weapons only
    pub damage_type: DamageType,                 //The type of damage the weapon deals
    pub defense: Option<i32>, //Some weapons (namely "gunlance" types) augment player defense; such weapons indicate that with this field
    pub deviation: Option<Deviation>, //For "light-bowgun" and "heavy-bowgun" weapons only
    pub elderseal: Option<Elderseal>, //The elderseal type attributed to the weapon
    pub phial_type: Option<PhialType>, //For "switch-axe" and "charge-blade" weapons only
    pub shelling_type: Option<ShellingType>, //For "gunlance" weapons only
    pub special_ammo: Option<SpecialAmmoType>, //For "light-bowgun" and "heavy-bowgun" weapons only
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeaponInfo {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_val: WeaponType,
    pub rarity: u32,
    pub attack: Attack,
    pub slots: Vec<Slot>,
    pub elements: Vec<WeaponElement>,
    pub crafting: WeaponCraftingInfo,
    pub assets: WeaponAssets,
    #[serde(default)]
    pub durability: Vec<WeaponSharpness>, // base at level 0, increasing handicraft levels
    pub attributes: WeaponAttributes,
    // internal details
    // ...
}

impl MhwWindowContents for WeaponInfo {
    fn build_window<'a>(
        &mut self,
        ui: &Ui<'a>,
        details: &mut GuiDetails,
        event_queue: &mut VecDeque<MhwEvent>,
    ) {
        ui.with_font(FONT_IDX_HEADER, || {
            let imstring = ImString::new(self.name.clone());
            ui.text_colored(rarity_color(self.rarity), &imstring);
        });
    }
}
