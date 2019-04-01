use super::mhw::common::fonts::*;
use imgui::*;
use std::convert::AsRef;

pub fn draw_table<LabelType: AsRef<str>, DataType: AsRef<str>>(
    ui: &Ui,
    title: &str,
    labels: &[LabelType],
    data: &[DataType],
) where
    DataType: std::fmt::Debug,
{
    // TODO: use a clip rect to make sure titles don't get drawn outside when narrow
    // TODO: don't draw it when its width is negative...?

    let text_line_height = ui.get_text_line_height_with_spacing();
    let window_padding = ui.imgui().style().window_padding;
    let content_avail = ui.get_content_region_avail();

    let draw_list = ui.get_window_draw_list();
    let frame_padding = ui.imgui().style().frame_padding;
    let cursor = ui.get_cursor_screen_pos();

    draw_list
        .add_rect(
            (cursor.0, cursor.1),
            (cursor.0 + content_avail.0, cursor.1 + text_line_height),
            (0.5, 0.1, 0.1, 1.0),
        )
        .filled(true)
        .rounding(3.0)
        .round_top_left(true)
        .round_top_right(true)
        .round_bot_left(false)
        .round_bot_right(false)
        .build();

    ui.indent(window_padding.x);
    ui.text(title);

    let col_count = labels.len();
    let row_count = data.len() / col_count;
    let child_height = text_line_height * (row_count + 1) as f32;
    let id_string = ImString::from(format!("table-view##{}", title));
    let child = ui
        .child_frame(&id_string, (content_avail.0, child_height))
        .show_borders(false);

    child.build(|| {
        // draw all the items!
        if row_count == 0 {
            ui.with_font(FONT_IDX_NORMAL, || {
                ui.text("<none>");
            });
            return;
        }

        ui.columns(col_count as i32, im_str!("inner_cols"), true);
        if col_count > 0 {
            // draw the label row
            ui.with_font(FONT_IDX_WINDOW_TITLE, || {
                for label in labels {
                    ui.text(label);
                    ui.next_column();
                }
            });

            ui.separator(); // TODO: Spills over, should roll our own
            ui.with_font(FONT_IDX_NORMAL, || {
                for row in 0..data.len() / col_count {
                    for idx in 0..col_count {
                        if let Some(datum) = data.get(row * col_count + idx) {
                            ui.text(datum.as_ref());
                        }
                        ui.next_column();
                    }
                }
            });
        }
    });
    ui.unindent(window_padding.x);

    let new_cursor = ui.get_cursor_screen_pos();
    draw_list
        .add_rect(
            (cursor.0, cursor.1),
            (new_cursor.0 + content_avail.0, new_cursor.1),
            ui.get_color(ImGuiCol::Border),
        )
        .rounding(3.0)
        .round_top_left(true)
        .round_top_right(true)
        .round_bot_left(true)
        .round_bot_right(true)
        .build();
    ui.set_cursor_screen_pos((new_cursor.0, new_cursor.1 + frame_padding.y));
}
