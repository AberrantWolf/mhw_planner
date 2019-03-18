use super::common::{Gui, ItemType};

use imgui::*;
use reqwest;
use reqwest::Url;
use serde::{Deserialize, Serialize};

// TODO: once this is refactored here, probably won't need pu fields

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResults {
    pub id: u64,
    pub name: String,
}

#[derive(Default, Debug)]
pub struct SearchState {
    pub find_type: ItemType,
    pub text: ImString,
    pub selected_item: i32,
    pub should_draw: bool,
    pub results: Vec<SearchResults>,
    // TODO: add filters
}

impl SearchState {
    pub fn query_api(&mut self) {
        self.results.clear();
        self.selected_item = -1;

        let url = Url::parse(
        format!(
            "https://mhw-db.com/armor?q={{\"name\":{{\"$like\":\"{}\"}}}}&p={{\"id\":true,\"name\":true,\"type\":true}}",
            urlencoding::encode(self.text.to_str())
            )
            .as_str(),
        )
        .unwrap();
        println!("{}", url.as_str());
        let mut result = match reqwest::get(url) {
            Ok(r) => r,
            Err(e) => {
                println!("Error querying API: {}", e);
                return;
            }
        };

        self.results = match result.json() {
            Ok(r) => r,
            Err(e) => {
                println!("Error converting API search into Vec: {}", e);
                vec![]
            }
        };
        println!("Found: {}", self.results.len());
    }
}

impl Gui for SearchState {
    fn layout(&mut self) {
        if !self.should_draw {
            return;
        }
    }
}
