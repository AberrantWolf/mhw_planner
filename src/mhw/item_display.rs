pub mod armor_view;

use super::common::{MhwEvent, MhwGui};
use armor_view::*;
use imgui::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum ItemId {
    Armor(u64),
}

#[derive(Debug)]
pub enum ItemData {
    Armor(ArmorInfo),
}

#[derive(Debug)]
pub struct ItemDisplayState {
    item_id: ItemId,
}

impl MhwGui for ItemDisplayState {
    fn layout<'a>(&mut self, ui: &Ui<'a>, event_queue: &mut VecDeque<MhwEvent>) {}
}
