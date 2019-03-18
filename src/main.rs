mod mhw;

use mhw::search;

use reqwest;
use reqwest::Url;

use urlencoding;

use serde::{Deserialize, Serialize};

use imgui::*;
mod support_glium;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

#[derive(Default, Debug)]
struct AppState {
    search_state: search::SearchState,
}

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

fn hello_world<'a>(ui: &Ui<'a>, state: &mut AppState) -> bool {
    let logical_size = ui.frame_size().logical_size;
    let mut window_size = (logical_size.0 as f32, logical_size.1 as f32);

    let mut menu_height = 0f32;

    // Main menu bar
    ui.with_font(2, || {
        ui.main_menu_bar(|| {
            menu_height = ui.get_window_size().1;
            window_size.1 -= menu_height;
            ui.menu(im_str!("File")).build(|| {
                ui.with_font(1, || if ui.menu_item(im_str!("Load Armor")).build() {});
            });
        });
    });

    ui.with_font(1, || {
        let window = ui
            .window(im_str!("root"))
            .position((0.0, menu_height), ImGuiCond::Always)
            .size(window_size, ImGuiCond::Always)
            .flags(ImGuiWindowFlags::NoDecoration | ImGuiWindowFlags::NoBringToFrontOnFocus);

        let mut build_func = || {
            ui.text(im_str!("Search: "));
            ui.same_line(0.0);
            if ui
                .input_text(im_str!(""), &mut state.search_state.text)
                .enter_returns_true(true)
                .build()
            {
                ui.set_keyboard_focus_here(-1);
                state.search_state.query_api();
            }
            ui.separator();
            let results_list = &state.search_state.results;
            let names_list_imstring = results_list
                .iter()
                .map(|res: &search::SearchResults| ImString::new(res.name.as_str()))
                .collect::<Vec<_>>();
            let ref_names = names_list_imstring
                .iter()
                .map(|name| name.as_ref())
                .collect::<Vec<_>>();
            ui.list_box(
                im_str!(""),
                &mut state.search_state.selected_item,
                ref_names.as_slice(),
                10,
            );
        };

        ui.with_style_var(StyleVar::WindowRounding(0.0), || {
            window.build(|| {
                ui.with_style_var(StyleVar::FrameRounding(4.0), || {
                    build_func();
                });
            });
        });
    });

    true
}

fn main() {
    let mut state = AppState::default();
    state.search_state.text = ImString::with_capacity(128);
    support_glium::run(
        "Monster [Helper] World".to_owned(),
        CLEAR_COLOR,
        |ui, _, _| hello_world(ui, &mut state),
    );
    // support_gfx::run("Monster [Helper] World".to_owned(), CLEAR_COLOR, |ui| {
    //     hello_world(ui, &mut state)
    // });
    // let response = match get_armor() {
    //     Ok(r) => r,
    //     Err(_) => panic!(),
    // };

    // println!("{:#?}", response[42]);

    //let html = mhw_scraper::load_page("https://mhworld.kiranico.com/monster/anjanath").unwrap();
    //let worked = mhw_scraper::scrape_monster(&html).unwrap();

    //println!("{:?}", worked);
}
