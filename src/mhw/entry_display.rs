use super::armor::*;
use super::common::{MhwEvent, MhwGui};
use crate::mhw::common::GuiDetails;
use crate::mhw::common::MhwWindowContents;
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
    fn layout<'a>(
        &mut self,
        ui: &Ui<'a>,
        details: &mut GuiDetails,
        event_queue: &mut VecDeque<MhwEvent>,
    ) {
        let logical_size = ui.frame_size().logical_size;
        let draw_cursor_pos = details.next_start_pos;
        let remaining_size = (
            logical_size.0 as f32 - draw_cursor_pos.0,
            logical_size.1 as f32 - draw_cursor_pos.1,
        );

        let title = match self {
            EntryDisplayState::None => im_str!("<Nothing Selected>"),
            EntryDisplayState::Armor(_) => im_str!("Armor Info"),
        };

        // TODO: Use bigger font here
        let window = ui
            .window(title)
            .position(draw_cursor_pos, ImGuiCond::Always)
            .size(remaining_size, ImGuiCond::Always);

        // TODO: Keep UI responsive with async getting data and placeholder UI until loaded
        match self {
            EntryDisplayState::None => window.build(|| {}),
            EntryDisplayState::Armor(ref mut armor) => {
                window.build(|| armor.build_window(ui, details, event_queue))
            } // TODO: add more as I build more entry types
        };
    }
}
