use crate::mhw::common::{fonts::*, rarity::rarity_color, GuiDetails, MhwEvent, MhwWindowContents};
use imgui::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ItemInfo {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub rarity: u32,
    pub carry_limit: u32,
    pub value: u32,
}

impl MhwWindowContents for ItemInfo {
    fn build_window<'a>(
        &mut self,
        ui: &Ui<'a>,
        _details: &mut GuiDetails,
        _event_queue: &mut VecDeque<MhwEvent>, // will need this when linking between objects
    ) {
        //=======================================
        // Name/ID
        ui.with_font(FONT_IDX_HEADER, || {
            let imstring = ImString::new(self.name.as_str());
            ui.text_colored(rarity_color(self.rarity), &imstring);
        });
        ui.with_font(FONT_IDX_MINI, || {
            let id_string = format!("id: [{}]", self.id);
            ui.text(id_string.as_str());
        });

        ui.columns(3, im_str!("item_misc"), true);
        // Description
        ui.with_font(FONT_IDX_WINDOW_TITLE, || {
            let text = im_str!("Description:");
            ui.text(text);
        });
        ui.with_font(FONT_IDX_NORMAL, || {
            let imstring = ImString::new(self.description.as_str());
            ui.text_wrapped(&imstring);
        });

        // Value
        ui.next_column();
        ui.with_font(FONT_IDX_WINDOW_TITLE, || {
            let text = im_str!("Value");
            ui.text(text);
        });
        ui.with_font(FONT_IDX_NORMAL, || {
            ui.text(self.value.to_string());
        });

        // Carry Limit
        ui.next_column();
        ui.with_font(FONT_IDX_WINDOW_TITLE, || {
            let text = im_str!("Carry Limit");
            ui.text(text);
        });
        ui.with_font(FONT_IDX_NORMAL, || {
            ui.text(self.carry_limit.to_string());
        });
    }
}
