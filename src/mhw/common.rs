use super::entry_display::EntryDisplayState;
use super::items::*;
use super::search::SearchState;
use imgui::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt::{self, Debug, Display};

pub mod fonts {
    pub const FONT_IDX_NORMAL: usize = 1;
    pub const FONT_IDX_WINDOW_TITLE: usize = 2;
    pub const FONT_IDX_MENU: usize = 3;
    pub const FONT_IDX_HEADER: usize = 4;
    pub const FONT_IDX_MINI: usize = 5;
}

#[macro_use]
pub mod macros {
    //
    // const_rgb_int!(NAME, R, G, B)
    //
    #[macro_export]
    macro_rules! const_rgb_int {
        ($name:ident, $r:expr, $g:expr, $b:expr) => {
            const $name: (f32, f32, f32, f32) =
                ($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0);
        };
    }
}

pub mod rarity {
    const_rgb_int!(RANK1, 191, 191, 191);
    const_rgb_int!(RANK2, 255, 255, 255);
    const_rgb_int!(RANK3, 194, 218, 126);
    const_rgb_int!(RANK4, 127, 191, 105);
    const_rgb_int!(RANK5, 122, 202, 205);
    const_rgb_int!(RANK6, 106, 126, 201);
    const_rgb_int!(RANK7, 184, 146, 216);
    const_rgb_int!(RANK8, 227, 174, 94);

    pub fn rarity_color(rarity: u32) -> (f32, f32, f32, f32) {
        match rarity {
            1 => RANK1,
            2 => RANK2,
            3 => RANK3,
            4 => RANK4,
            5 => RANK5,
            6 => RANK6,
            7 => RANK7,
            8 => RANK8,
            _ => (1.0, 1.0, 0.0, 1.0),
        }
    }
}

use fonts::*;

#[derive(Debug)]
pub struct GuiDetails {
    pub next_start_pos: (f32, f32),
    pub draw_filter_window: bool,
}

pub trait MhwGui {
    fn layout<'a>(
        &mut self,
        ui: &Ui<'a>,
        details: &mut GuiDetails,
        event_queue: &mut VecDeque<MhwEvent>,
    );
}

pub trait MhwWindowContents {
    fn build_window<'a>(
        &mut self,
        ui: &Ui<'a>,
        details: &mut GuiDetails,
        event_queue: &mut VecDeque<MhwEvent>,
    );
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CraftingCost {
    pub quantity: i32,
    pub item: ItemInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Slot {
    pub rank: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SkillRank {
    pub id: i32,
    pub level: i32,
    pub description: String,
    pub skill: i32,
    pub skill_name: String,
    pub modifiers: SkillRankModifiers,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SkillRankModifiers {
    pub affinity: f32,
    pub attack: i32,
    pub damage_fire: i32,
    pub damage_water: i32,
    pub damage_ice: i32,
    pub damage_thunder: i32,
    pub damage_dragon: i32,
    pub defense: i32,
    pub health: i32,
    pub sharpness_bonus: i32,
    pub resist_all: i32,
    pub resist_fire: i32,
    pub resist_water: i32,
    pub resist_ice: i32,
    pub resist_thunder: i32,
    pub resist_dragon: i32,
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
    gui_details: GuiDetails,
}

impl AppState {
    pub fn should_quit(&self) -> bool {
        self.quit_requested
    }

    pub fn process_events(&mut self) {
        for evt in self.event_list.drain(..) {
            match evt {
                MhwEvent::ShowState(state) => {
                    // Set the current display state to the one we just loaded
                    self.entry_display_state = state;
                }
            }
        }
    }

    pub fn layout<'a>(&mut self, ui: &Ui<'a>) {
        self.gui_details.next_start_pos = (0.0, 0.0);

        ui.with_font(FONT_IDX_MENU, || {
            ui.main_menu_bar(|| {
                self.gui_details.next_start_pos.1 = ui.get_window_size().1;
                ui.menu(im_str!("File")).build(|| {
                    ui.with_font(1, || {
                        if ui.menu_item(im_str!("Quit")).build() {
                            self.quit_requested = true
                        }
                    });
                });
            });
        });

        ui.with_font(FONT_IDX_NORMAL, || {
            self.search_state
                .layout(&ui, &mut self.gui_details, &mut self.event_list);
            self.entry_display_state
                .layout(&ui, &mut self.gui_details, &mut self.event_list);
            if self.gui_details.draw_filter_window {
                self.search_state.layout_filter_window(
                    &ui,
                    &mut self.gui_details,
                    &mut self.event_list,
                );
            }
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
            gui_details: GuiDetails {
                next_start_pos: (0.0, 0.0),
                draw_filter_window: false,
            },
        }
    }
}
