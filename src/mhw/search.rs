use super::armor::ArmorInfo;
use super::common::{GuiDetails, MhwEvent, MhwGui};
use super::entry_display::EntryDisplayState;
use super::items::ItemInfo;
use super::query::*;
use super::query_filters;
use super::weapons::WeaponInfo;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::fmt;

use imgui::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const SEARCH_WINDOW_WIDTH: f32 = 240f32;

//
// Search Category
//
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum SearchCategory {
    Armor = 0,
    Weapons,
    Items,
    MAX,
}

impl Default for SearchCategory {
    fn default() -> Self {
        SearchCategory::Armor
    }
}

impl fmt::Display for SearchCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchCategory::Armor => write!(f, "armor"),
            SearchCategory::Weapons => write!(f, "weapons"),
            SearchCategory::Items => write!(f, "items"),
            _ => write!(f, "ERROR"),
        }
    }
}

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
    filters: Vec<QueryFilter>,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            search_type: Default::default(),
            text: ImString::with_capacity(128),
            selected_item: -1,
            should_draw: true,
            results: vec![],
            filters: vec![],
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
            SearchCategory::Items => {
                let found: Result<Vec<ItemInfo>, MHWQueryError> = query.execute_mhw_query();
                match found {
                    Ok(mut f) => {
                        if !f.is_empty() {
                            EntryDisplayState::Item(f.remove(0))
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

impl SearchState {
    pub fn layout_filter_window<'a>(
        &mut self,
        ui: &Ui<'a>,
        details: &mut GuiDetails,
        _event_queue: &mut VecDeque<MhwEvent>,
    ) {
        let logical_size = ui.frame_size().logical_size;
        let draw_cursor_pos = details.next_start_pos;
        let window_size = (
            ((logical_size.0 as f32 - draw_cursor_pos.0) * 0.67),
            ((logical_size.1 as f32 - draw_cursor_pos.1) * 0.5),
        );
        let window = ui
            .window(im_str!("Filter"))
            .position(draw_cursor_pos, ImGuiCond::Always)
            .size(window_size, ImGuiCond::Always)
            .flags(ImGuiWindowFlags::NoDecoration);
        window.build(|| {
            let categories = query_filters::get_filter_categories(&self.search_type);
            let ref_names = categories
                .iter()
                .map(std::convert::AsRef::as_ref)
                .collect::<Vec<_>>();
            //for mut filter in &self.filters {
            // draw the stuff
            let mut idx = 0;
            if ui.combo(
                im_str!("##filter_combo"),
                &mut idx,
                ref_names.as_slice(),
                SearchCategory::MAX as i32,
            ) {}
            //}
        });
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
            .flags(ImGuiWindowFlags::NoDecoration);

        let mut build_func = || {
            // select category
            let mut idx = self.search_type as i32;
            if ui.combo(
                im_str!("##category_combo"),
                &mut idx,
                &[im_str!("Armor"), im_str!("Weapons"), im_str!("Items")],
                SearchCategory::MAX as i32,
            ) {}
            ui.same_line(0.0);
            ui.with_item_width(-1.0, || {
                if ui.button(im_str!("Filter..."), (-1.0, 0.0)) {
                    details.draw_filter_window = !details.draw_filter_window;
                }
            });
            if let Some(result) = SearchCategory::from_i32(idx) {
                self.search_type = result;
            }

            // type name filter string
            ui.text(im_str!("Name: "));
            ui.same_line(0.0);
            ui.with_item_width(-1.0, || {
                if ui
                    .input_text(im_str!("###search_input"), &mut self.text)
                    .enter_returns_true(true)
                    .build()
                {
                    ui.set_keyboard_focus_here(-1);
                    self.query_api();
                }
            });
            ui.separator();
            let results_list = &self.results;
            let names_list_imstring = results_list
                .iter()
                .map(|res: &SearchResults| ImString::new(res.name.as_str()))
                .collect::<Vec<_>>();
            let ref_names = names_list_imstring
                .iter()
                .map(std::convert::AsRef::as_ref)
                .collect::<Vec<_>>();

            let available_space = ui.get_content_region_avail();
            ui.with_item_width(-1.0, || {
                let did_change = ui.list_box(
                    im_str!("###search_results"),
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
