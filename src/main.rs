mod mhw;

use mhw::common::{AppState, MhwGui};

use serde::{Deserialize, Serialize};

mod support_glium;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

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
    name: String,
    #[serde(rename = "type")]
    armor_type: ArmorType,
}

fn main() {
    let mut state = AppState::default();
    support_glium::run(
        "Monster [Helper] World".to_owned(),
        CLEAR_COLOR,
        |ui, _, _| {
            state.layout(&ui);
            state.process_events();
            !state.should_quit()
        },
    );

    //let html = mhw_scraper::load_page("https://mhworld.kiranico.com/monster/anjanath").unwrap();
    //let worked = mhw_scraper::scrape_monster(&html).unwrap();

    //println!("{:?}", worked);
}
