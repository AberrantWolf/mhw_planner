use super::entry_display::EntryDisplayState;
use super::item::*;
use super::search::SearchState;
use imgui::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub trait MhwGui {
    fn layout<'a>(&mut self, ui: &Ui<'a>, event_queue: &mut VecDeque<MhwEvent>);
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Element {
    Fire,
    Water,
    Ice,
    Thunder,
    Dragon,
    Blast,
    Poison,
    Sleep,
    Paralysis,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CraftingCost {
    quantity: i32,
    item: Item,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Slot {
    rank: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SkillRank {
    id: i32,
    level: i32,
    description: String,
    skill: i32,
    skill_name: String,
    modifiers: SkillRankModifiers,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SkillRankModifiers {
    affinity: f32,
    attack: i32,
    damage_fire: i32,
    damage_water: i32,
    damage_ice: i32,
    damage_thunder: i32,
    damage_dragon: i32,
    defense: i32,
    health: i32,
    sharpness_bonus: i32,
    resist_all: i32,
    resist_fire: i32,
    resist_water: i32,
    resist_ice: i32,
    resist_thunder: i32,
    resist_dragon: i32,
}

#[derive(Debug)]
pub enum MhwEvent {
    ShowState(EntryDisplayState),
}

#[derive(Debug)]
pub struct AppState {
    quit_requested: bool,
    search_state: SearchState,
    entry_display_state: EntryDisplayState,
    event_list: VecDeque<MhwEvent>,
}

impl AppState {
    pub fn should_quit(&self) -> bool {
        self.quit_requested
    }

    pub fn process_events(&mut self) {
        for evt in self.event_list.drain(..) {
            match &evt {
                MhwEvent::ShowState(_id) => {
                    println!("Processing event: {:?}", evt);
                    // create search info
                    // match result
                    // set the
                }
            }
        }
    }

    pub fn layout<'a>(&mut self, ui: &Ui<'a> /*, state: &mut AppState*/) {
        let mut menu_height = 0f32;

        ui.with_font(2, || {
            ui.main_menu_bar(|| {
                menu_height = ui.get_window_size().1;
                ui.menu(im_str!("File")).build(|| {
                    ui.with_font(1, || {
                        if ui.menu_item(im_str!("Quit")).build() {
                            self.quit_requested = true
                        }
                    });
                });
            });
        });

        ui.with_font(1, || {
            // this causes the debug window to appear... try to stop that
            ui.set_cursor_pos((0f32, menu_height));

            self.search_state.layout(&ui, &mut self.event_list);
            // FUTURE: we'll need a state of viewing an item or plan or something
            // else and choose what to draw next based on that state.
            self.entry_display_state.layout(&ui, &mut self.event_list);
        });
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            quit_requested: false,
            search_state: Default::default(),
            entry_display_state: Default::default(),
            event_list: Default::default(),
        }
    }
}
