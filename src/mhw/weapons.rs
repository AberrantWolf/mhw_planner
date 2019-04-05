use super::common::{
    fonts::*, rarity::*, CraftingCost, Element, GuiDetails, MhwEvent, MhwWindowContents, Slot,
};
use crate::widgets::table_view::*;
use imgui::*;
use onig::*;
use serde::de;
use serde::de::Visitor;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
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
        match self {
            WeaponType::GreatSword => write!(f, "Great Sword"),
            WeaponType::LongSword => write!(f, "Long Sword"),
            WeaponType::SwordAndShield => write!(f, "Sword and Shield"),
            WeaponType::DualBlades => write!(f, "Dual Blades"),
            WeaponType::Hammer => write!(f, "Hammer"),
            WeaponType::HuntingHorn => write!(f, "Hunting Horn"),
            WeaponType::Lance => write!(f, "Lance"),
            WeaponType::Gunlance => write!(f, "Gunlance"),
            WeaponType::SwitchAxe => write!(f, "Switch Axe"),
            WeaponType::ChargeBlade => write!(f, "Charge Blade"),
            WeaponType::InsectGlaive => write!(f, "Insect Glaive"),
            WeaponType::LightBowgun => write!(f, "Light Bowgun"),
            WeaponType::HeavyBowgun => write!(f, "Heavy Bowgun"),
            WeaponType::Bow => write!(f, "Bow"),
        }
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
    pub red: f32,
    pub orange: f32,
    pub yellow: f32,
    pub green: f32,
    pub blue: f32,
    pub white: f32,
}

impl WeaponSharpness {
    pub fn draw(&self, ui: &Ui) {
        static SHARPNESS_HEIGHT: f32 = 7.0;
        static OUTLINE_COLOR: (f32, f32, f32, f32) = (0.2, 0.2, 0.2, 1.0);

        let start_pos = ui.get_cursor_screen_pos();
        let content_avail = ui.get_content_region_avail();
        let end_pos = (
            start_pos.0 + content_avail.0,
            start_pos.1 + SHARPNESS_HEIGHT,
        );
        let draw_list = ui.get_window_draw_list();

        //               sharpness   total_width
        // total_width * --------- = ----------- * sharpness
        //                  400          400
        let width_mod = content_avail.0 / 400.0;

        const_rgb_int!(SHARP_RED, 217, 44, 44);
        const_rgb_int!(SHARP_ORG, 217, 102, 44);
        const_rgb_int!(SHARP_YEL, 217, 209, 44);
        const_rgb_int!(SHARP_GRN, 112, 217, 44);
        const_rgb_int!(SHARP_BLU, 44, 134, 217);
        const_rgb_int!(SHARP_WHT, 255, 255, 255);

        let mut next_start = start_pos;
        let mut next_end = start_pos;

        macro_rules! draw_next_segment {
            ($segment:ident, $color:expr) => {
                next_start = (next_end.0, next_start.1);
                next_end = (
                    next_start.0 + self.$segment as f32 * width_mod,
                    next_start.1 + SHARPNESS_HEIGHT,
                );
                draw_list
                    .add_rect(next_start, next_end, $color)
                    .filled(true)
                    .build();
            };
        }
        draw_next_segment!(red, SHARP_RED);
        draw_next_segment!(orange, SHARP_ORG);
        draw_next_segment!(yellow, SHARP_YEL);
        draw_next_segment!(green, SHARP_GRN);
        draw_next_segment!(blue, SHARP_BLU);
        draw_next_segment!(white, SHARP_WHT);

        draw_list
            .add_rect(start_pos, end_pos, OUTLINE_COLOR)
            .filled(false)
            .build();

        ui.set_cursor_screen_pos((start_pos.0, end_pos.1));
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum AmmoType {
    Normal,
    Flaming,
    Piercing,
    Water,
    Spread,
    Freeze,
    Sticky,
    Thunder,
    Cluster,
    Dragon,
    Recover,
    Slicing,
    Poison,
    Wyvern,
    Paralysis,
    Demon,
    Sleep,
    Armor,
    Exhaust,
    Tranq,
}

impl Display for AmmoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PhialType {
    Impact,
    Element,
    Power,
    PowerElement,
    Dragon(i32),
    Exhaust(i32),
    Paralysis(i32),
    Poison(i32),
}

fn calc_power(parts: &Captures) -> Option<i32> {
    if let Some(result) = parts.at(2) {
        if let Ok(val) = result.parse::<i32>() {
            Some(val)
        } else {
            //println!("{:?}", parts);
            None
        }
    } else {
        //println!("{:?}", parts);
        None
    }
}

impl<'de> Deserialize<'de> for PhialType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        const FIELDS: &[&str] = &[
            "Impact",
            "Element",
            "Power",
            "Power Element",
            "Dragon XXX",
            "Exhaust XXX",
            "Paralysis XXX",
            "Poison XXX",
        ];

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = PhialType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`Normal` or `Long` or `Wide`")
            }

            fn visit_str<E>(self, value: &str) -> Result<PhialType, E>
            where
                E: de::Error,
            {
                println!("Parsing value: {}", value);
                // create the regex
                // (\D+)[\s]?(\d+)?
                // ([^0-9]+)([0-9]+)?
                let re = if let Ok(result) = Regex::new(r"(\D+)\s?(?!\d+)\D?(\d+)?") {
                    result
                } else {
                    return Err(de::Error::unknown_field(value, FIELDS));
                };

                // split the string
                let parts = if let Some(result) = re.captures(value) {
                    result
                } else {
                    //println!("{:?}", re.captures(value));
                    return Err(de::Error::unknown_field(value, FIELDS));
                };

                let type_name = if let Some(result) = parts.at(1) {
                    result
                } else {
                    //println!("{:?}", parts);
                    return Err(de::Error::unknown_field(value, FIELDS));
                };

                match type_name {
                    "impact" => Ok(PhialType::Impact),
                    "impact phial" => Ok(PhialType::Impact),
                    "element" => Ok(PhialType::Element),
                    "power" => Ok(PhialType::Power),
                    "power element" => Ok(PhialType::PowerElement),
                    "power element phial" => Ok(PhialType::PowerElement),
                    "dragon" => {
                        if let Some(power) = calc_power(&parts) {
                            Ok(PhialType::Dragon(power))
                        } else {
                            println!("{:?}", parts);
                            Err(de::Error::unknown_field(value, FIELDS))
                        }
                    }
                    "exhaust" => {
                        if let Some(power) = calc_power(&parts) {
                            Ok(PhialType::Exhaust(power))
                        } else {
                            println!("{:?}", parts);
                            Err(de::Error::unknown_field(value, FIELDS))
                        }
                    }
                    "paralysis" => {
                        if let Some(power) = calc_power(&parts) {
                            Ok(PhialType::Paralysis(power))
                        } else {
                            println!("{:?}", parts);
                            Err(de::Error::unknown_field(value, FIELDS))
                        }
                    }
                    "poison" => {
                        if let Some(power) = calc_power(&parts) {
                            Ok(PhialType::Poison(power))
                        } else {
                            println!("{:?}", parts);
                            Err(de::Error::unknown_field(value, FIELDS))
                        }
                    }
                    _ => {
                        println!("{:?}", parts);
                        Err(de::Error::unknown_field(value, FIELDS))
                    }
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

impl Display for PhialType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Debug)]
pub enum ShellingType {
    Normal(i32),
    Long(i32),
    Wide(i32),
}

impl<'de> Deserialize<'de> for ShellingType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        const FIELDS: &[&str] = &["Normal LvN", "Long LvN", "Wide LvN"];

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = ShellingType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`Normal` or `Long` or `Wide`")
            }

            fn visit_str<E>(self, value: &str) -> Result<ShellingType, E>
            where
                E: de::Error,
            {
                let mut parts = value.split(' ');
                let label = parts.next();
                let lvl = parts.next();
                let lvl_num = if let Some(lvl_str) = lvl {
                    let lvl_num_result = lvl_str[2..].parse::<i32>();
                    if let Ok(val) = lvl_num_result {
                        val
                    } else {
                        return Err(de::Error::unknown_field(value, FIELDS));
                    }
                } else {
                    return Err(de::Error::unknown_field(value, FIELDS));
                };

                match label {
                    Some("Normal") => Ok(ShellingType::Normal(lvl_num)),
                    Some("Long") => Ok(ShellingType::Long(lvl_num)),
                    Some("Wide") => Ok(ShellingType::Wide(lvl_num)),
                    _ => Err(de::Error::unknown_field(value, FIELDS)),
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
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
        write!(f, "Wyvern {}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeaponAttributes {
    pub ammo_capacities: Option<BTreeMap<AmmoType, Vec<i32>>>, //For "light-bowgun" and "heavy-bowgun" weapons only
    pub affinity: Option<i32>,                                 //The affinity of the weapon
    pub boost_type: Option<BoostType>,                         //For "insect-glaive" weapons only
    pub coatings: Option<Vec<Coating>>,                        //For "bow" weapons only
    pub damage_type: Option<DamageType>,                       //The type of damage the weapon deals
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
    //#[serde(deserialize_with = "deserialize_struct_case_insensitive")]
    pub attributes: WeaponAttributes,

    // internal details
    #[serde(skip)]
    crafting_cache: SimpleTableDataModel,
    #[serde(skip)]
    upgrade_cache: SimpleTableDataModel,
    #[serde(skip)]
    attributes_cache: SimpleTableDataModel,
}

impl WeaponInfo {
    fn crafting_data(&mut self) -> &TableDataModel {
        if self.crafting_cache.is_empty() {
            self.crafting_cache.set_columns(2);
            let mats = &self.crafting.crafting_materials;
            for cost in mats {
                self.crafting_cache.push(cost.item.name.clone());
                self.crafting_cache.push(cost.quantity.to_string());
            }
        }
        &self.crafting_cache
    }

    fn upgrading_data(&mut self) -> &TableDataModel {
        if self.upgrade_cache.is_empty() {
            self.upgrade_cache.set_columns(2);
            let mats = &self.crafting.upgrade_materials;
            for cost in mats {
                self.upgrade_cache.push(cost.item.name.clone());
                self.upgrade_cache.push(cost.quantity.to_string());
            }
        }
        &self.upgrade_cache
    }

    fn attribute_data(&mut self) -> &TableDataModel {
        if self.attributes_cache.is_empty() {
            self.attributes_cache.set_columns(2);
            let attribs = &self.attributes;
            if let Some(ref ammo_caps_map) = attribs.ammo_capacities {
                self.attributes_cache.push("Ammo Capacities".to_owned());
                self.attributes_cache.push("".to_owned());

                for (ammo_type, caps) in ammo_caps_map {
                    let mut has_caps = false;
                    let mut cap_strings: Vec<String> = vec![];
                    for (idx, cap) in caps.iter().enumerate() {
                        if *cap > 0 {
                            cap_strings.push(format!("  Lv{}: {}", idx + 1, cap));
                            cap_strings.push("".to_owned());
                            has_caps = true;
                        }
                    }
                    cap_strings.pop();

                    if has_caps {
                        self.attributes_cache.push(format!("    {}", ammo_type));
                        self.attributes_cache.append(cap_strings);
                    }
                }
            }
            if let Some(ref attr) = attribs.affinity {
                self.attributes_cache.push("Affinity".to_owned());
                self.attributes_cache.push(attr.to_string());
            }
            if let Some(ref attr) = attribs.boost_type {
                self.attributes_cache.push("Boost Type".to_owned());
                self.attributes_cache.push(attr.to_string());
            }
            if let Some(ref attr) = attribs.coatings {
                self.attributes_cache.push("Coatings".to_owned());
                for coating in attr {
                    self.attributes_cache.push(format!("{}", coating));
                    self.attributes_cache.push("".to_owned());
                }
                self.attributes_cache.pop();
            }
            if let Some(ref attr) = attribs.damage_type {
                self.attributes_cache.push("Damage Type".to_owned());
                self.attributes_cache.push(attr.to_string());
            }
            if let Some(ref attr) = attribs.defense {
                self.attributes_cache.push("Defense".to_owned());
                self.attributes_cache.push(attr.to_string());
            }
            if let Some(ref attr) = attribs.deviation {
                self.attributes_cache.push("Deviation".to_owned());
                self.attributes_cache.push(format!("{}", attr));
            }
            if let Some(ref attr) = attribs.elderseal {
                self.attributes_cache.push("Elderseal".to_owned());
                self.attributes_cache.push(attr.to_string());
            }
            if let Some(ref attr) = attribs.phial_type {
                self.attributes_cache.push("Phial Type".to_owned());
                self.attributes_cache.push(format!("{}", attr));
            }
            if let Some(ref attr) = attribs.shelling_type {
                self.attributes_cache.push("Shelling Type".to_owned());
                self.attributes_cache.push(format!("{}", attr));
            }
            if let Some(ref attr) = attribs.special_ammo {
                self.attributes_cache.push("Special Ammo".to_owned());
                self.attributes_cache.push(attr.to_string());
            }
        }
        &self.attributes_cache
    }
}

impl MhwWindowContents for WeaponInfo {
    fn build_window<'a>(
        &mut self,
        ui: &Ui<'a>,
        _details: &mut GuiDetails,
        _event_queue: &mut VecDeque<MhwEvent>,
    ) {
        //=======================================
        // Name/ID
        ui.with_font(FONT_IDX_HEADER, || {
            let imstring = ImString::new(self.name.as_str());
            ui.text_colored(rarity_color(self.rarity), &imstring);
        });
        ui.with_font(FONT_IDX_MINI, || {
            let id_string = format!("id: [{}]", self.id);
            ui.text(id_string.as_str());
            ui.same_line(0.0);
            ui.text(self.type_val.to_string());
        });

        //=======================================
        // Core Stats
        ui.columns(4, im_str!("armor_stats"), true);
        // Attack
        ui.with_font(FONT_IDX_WINDOW_TITLE, || {
            let text = im_str!("Attack");
            ui.text(text);
        });
        ui.with_font(FONT_IDX_NORMAL, || {
            ui.text(format!("{}/({})", self.attack.display, self.attack.raw));
        });

        // Elements
        ui.next_column();
        ui.with_font(FONT_IDX_WINDOW_TITLE, || {
            let text = im_str!("Elements");
            ui.text(text);
        });
        for elem in &self.elements {
            ui.text(format!("{}: ", elem.elememt));
            ui.same_line(0.0);
            ui.with_font(FONT_IDX_NORMAL, || {
                ui.text(format!("{}", elem.damage));
            });
        }

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

        // Durability
        ui.next_column();
        ui.with_font(FONT_IDX_WINDOW_TITLE, || {
            let text = im_str!("Durability");
            ui.text(text);
        });
        for durability in &self.durability {
            durability.draw(&ui);
        }
        {
            let cursor = ui.get_cursor_screen_pos();
            ui.set_cursor_screen_pos((cursor.0, cursor.1 + ui.imgui().style().frame_padding.y));
        }

        //=======================================
        // Lists section
        ui.columns(2, im_str!("armor_attribs"), true);
        ui.separator();
        // Maybe don't need elements, as there seems to only ever be 0/1 of them.
        //draw_table(ui, "Elements", &ELEMENTS_COLUMNS, self.elements_data());
        if self.crafting.craftable {
            draw_table(ui, "Crafting", self.crafting_data());
        }

        if let Some(previous) = self.crafting.previous {
            ui.text("Upgrade From: ");
            ui.same_line(0.0);
            ui.with_font(FONT_IDX_NORMAL, || {
                // TODO: once we cache item names, fetch the previous item name from ID
                ui.text(format!("id [{}]", previous));
            });
            draw_table(ui, "Required", self.upgrading_data());
        }

        ui.next_column();
        draw_table(ui, "Attributes", self.attribute_data());
    }
}
