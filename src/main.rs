mod mhw;

use mhw::common::AppState;

mod support_glium;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

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
