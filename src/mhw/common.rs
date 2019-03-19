use super::search::SearchState;
use imgui::*;

pub trait MhwGui {
    fn layout<'a>(&mut self, ui: &Ui<'a> /*, state: &mut AppState*/);
}

#[derive(Debug)]
pub enum ItemType {
    Armor,
    Weapon,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Armor
    }
}

#[derive(Debug)]
pub struct AppState {
    quit_requested: bool,
    search_state: SearchState,
}

impl AppState {
    pub fn should_quit(&self) -> bool {
        self.quit_requested
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            quit_requested: false,
            search_state: Default::default(),
        }
    }
}

impl MhwGui for AppState {
    fn layout<'a>(&mut self, ui: &Ui<'a> /*, state: &mut AppState*/) {
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

            self.search_state.layout(&ui);
        });
    }
}
