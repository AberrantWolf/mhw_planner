use super::armor::ArmorInfo;
use super::common::{MhwEvent, MhwGui};
use super::entry_display::EntryDisplayState;
use super::query::*;
use super::weapon::WeaponInfo;
use crate::mhw::common::GuiDetails;
use num_traits::FromPrimitive;

use imgui::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

// TODO: once this is refactored here, probably won't need pub fields

const SEARCH_WINDOW_WIDTH: f32 = 240f32;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResults {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct SearchState {
    pub search_type: SearchCategory,
    pub text: ImString,
    pub selected_item: i32,
    pub should_draw: bool,
    pub results: Vec<SearchResults>,
    // TODO: add filters
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            search_type: Default::default(),
            text: ImString::with_capacity(128),
            selected_item: -1,
            should_draw: true,
            results: vec![],
        }
    }
}

impl SearchState {
    pub fn query_api(&mut self) {
        self.results.clear();
        self.selected_item = -1;

        let found = QueryInfo::find_ids(self.text.to_str(), self.search_type).execute_mhw_query();
        self.results = match found {
            Ok(v) => v,
            Err(e) => {
                println!("API Query failed: {}", e);
                return;
            }
        };

        println!("Found: {}", self.results.len());
    }

    fn get_entry_for_selection(&self) -> EntryDisplayState {
        let id_string = if self.selected_item >= 0 {
            let idx = self.selected_item as usize;
            format!("{}", self.results[idx].id)
        } else {
            return EntryDisplayState::None;
        };
        let filter = QueryFilter::new("id".to_owned(), QueryFilterType::Exact(id_string));
        let query = QueryInfo::find_category(self.search_type).with_filter(filter);

        match self.search_type {
            SearchCategory::Armor => {
                let found: Result<Vec<ArmorInfo>, MHWQueryError> = query.execute_mhw_query();
                match found {
                    Ok(mut f) => {
                        if !f.is_empty() {
                            EntryDisplayState::Armor(f.remove(0))
                        } else {
                            EntryDisplayState::None
                        }
                    }
                    Err(e) => {
                        println!("Error retrieving entry: {}", e);
                        EntryDisplayState::None
                    }
                }
            }
            SearchCategory::Weapons => {
                let found: Result<Vec<WeaponInfo>, MHWQueryError> = query.execute_mhw_query();
                match found {
                    Ok(mut f) => {
                        if !f.is_empty() {
                            EntryDisplayState::Weapon(f.remove(0))
                        } else {
                            EntryDisplayState::None
                        }
                    }
                    Err(e) => {
                        println!("Error retrieving entry: {}", e);
                        EntryDisplayState::None
                    }
                }
            }
            _ => EntryDisplayState::None,
        }
    }
}

impl MhwGui for SearchState {
    fn layout<'a>(
        &mut self,
        ui: &Ui<'a>,
        details: &mut GuiDetails,
        event_queue: &mut VecDeque<MhwEvent>,
    ) {
        if !self.should_draw {
            return;
        }
        let logical_size = ui.frame_size().logical_size;
        let draw_cursor_pos = details.next_start_pos;
        let window_size = (logical_size.0 as f32, logical_size.1 as f32);
        details.next_start_pos.0 = SEARCH_WINDOW_WIDTH;

        let window = ui
            .window(im_str!("Search..."))
            .position(draw_cursor_pos, ImGuiCond::Always)
            .size(
                (SEARCH_WINDOW_WIDTH, window_size.1 - draw_cursor_pos.1),
                ImGuiCond::Always,
            )
            .flags(ImGuiWindowFlags::NoDecoration);;

        let mut build_func = || {
            // select category
            let mut idx = self.search_type as i32;
            if ui.combo(
                im_str!("##category_combo"),
                &mut idx,
                &[im_str!("Armor"), im_str!("Weapon")],
                SearchCategory::MAX as i32,
            ) {}
            if let Some(result) = SearchCategory::from_i32(idx) {
                self.search_type = result;
            }

            // type name filter string
            ui.text(im_str!("Name: "));
            ui.same_line(0.0);
            if ui
                .input_text(im_str!(""), &mut self.text)
                .enter_returns_true(true)
                .build()
            {
                ui.set_keyboard_focus_here(-1);
                self.query_api();
            }
            ui.separator();
            let results_list = &self.results;
            let names_list_imstring = results_list
                .iter()
                .map(|res: &SearchResults| ImString::new(res.name.as_str()))
                .collect::<Vec<_>>();
            let ref_names = names_list_imstring
                .iter()
                .map(|name| name.as_ref())
                .collect::<Vec<_>>();

            let available_space = ui.get_content_region_avail();
            ui.with_item_width(-1.0, || {
                let did_change = ui.list_box(
                    im_str!("search_results"),
                    &mut self.selected_item,
                    ref_names.as_slice(),
                    ((available_space.1 - ui.imgui().style().frame_padding.y * 2.0)
                        / ui.get_text_line_height_with_spacing()) as i32,
                );

                if did_change {
                    let found = self.get_entry_for_selection();
                    event_queue.push_back(MhwEvent::ShowState(found));
                }
            });
        };

        window.build(|| {
            ui.with_style_var(StyleVar::FrameRounding(4.0), || {
                build_func();
            });
        });
    }
}
