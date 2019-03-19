use super::common::{AppState, ItemType, MhwGui};

use imgui::*;
use reqwest;
use reqwest::Url;
use serde::{Deserialize, Serialize};

// TODO: once this is refactored here, probably won't need pub fields

const SEARCH_WINDOW_WIDTH: f32 = 240f32;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResults {
    pub id: u64,
    pub name: String,
}

#[derive(Debug)]
pub struct SearchState {
    pub find_type: ItemType,
    pub text: ImString,
    pub selected_item: i32,
    pub should_draw: bool,
    pub results: Vec<SearchResults>,
    // TODO: add filters
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            find_type: Default::default(),
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

impl MhwGui for SearchState {
    fn layout<'a>(&mut self, ui: &Ui<'a> /*, state: &mut AppState*/) {
        if !self.should_draw {
            return;
        }
        let logical_size = ui.frame_size().logical_size;
        let draw_cursor_pos = ui.get_cursor_pos();
        let window_size = (logical_size.0 as f32, logical_size.1 as f32);

        let window = ui
            .window(im_str!("Search..."))
            .position(draw_cursor_pos, ImGuiCond::Always)
            .size(
                (SEARCH_WINDOW_WIDTH, window_size.1 - draw_cursor_pos.1),
                ImGuiCond::Always,
            )
            .flags(ImGuiWindowFlags::NoDecoration);;

        let mut build_func = || {
            ui.text(im_str!("Search: "));
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
                ui.list_box(
                    im_str!("search_results"),
                    &mut self.selected_item,
                    ref_names.as_slice(),
                    ((available_space.1 - ui.imgui().style().frame_padding.y * 2.0)
                        / ui.get_text_line_height_with_spacing()) as i32,
                );
            });
        };

        // ui.with_style_var(StyleVar::WindowRounding(0.0), || {
        window.build(|| {
            ui.with_style_var(StyleVar::FrameRounding(4.0), || {
                build_func();
            });
        });
        // });
    }
}
