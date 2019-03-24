use super::armor::*;
use super::common::{MhwEvent, MhwGui};
use imgui::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum EntryDisplayState {
    None,
    Armor(ArmorInfo),
}

impl Default for EntryDisplayState {
    fn default() -> Self {
        EntryDisplayState::None
    }
}

impl MhwGui for EntryDisplayState {
    fn layout<'a>(&mut self, ui: &Ui<'a>, event_queue: &mut VecDeque<MhwEvent>) {
        // TODO: Keep UI responsive with async getting data and placeholder UI until loaded
        match self {
            EntryDisplayState::None => {}
            EntryDisplayState::Armor(ref mut armor) => {
                armor.layout(ui, event_queue);
            }
        };
    }
}
