mod mhw_scraper;

use reqwest;

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ArmorType {
    Head,
    Chest,
    Gloves,
    Waist,
    Legs,
}

#[derive(Serialize, Deserialize, Debug)]
struct Armor {
    id: u32,
    slug: String,
    name: String,
    #[serde(rename = "type")]
    armor_type: ArmorType,
}

fn get_armor() -> Result<Vec<Armor>, reqwest::Error> {
    let mut result = reqwest::get("https://mhw-db.com/armor")?;

    result.json()
}

fn main() {
    // let response = match get_armor() {
    //     Ok(r) => r,
    //     Err(_) => panic!(),
    // };

    // println!("{:#?}", response[42]);

    let html = mhw_scraper::load_page("https://mhworld.kiranico.com/monster/anjanath").unwrap();
    let worked = mhw_scraper::scrape_monster(&html).unwrap();

    println!("{:?}", worked);
}
