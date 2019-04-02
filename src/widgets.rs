use super::mhw::common::fonts::*;
use imgui::*;
use std::fmt::Debug;

pub trait TableDataModel {
    fn is_empty(&self) -> bool;
    fn col_count(&self) -> usize;
    fn row_count(&self) -> usize;
    fn draw_cell(&self, ui: &Ui, col: usize, row: usize);
}

#[derive(Debug)]
pub struct SimpleTableDataModel {
    col_count: usize,
    data: Vec<String>,
}

impl SimpleTableDataModel {
    pub fn new(col_count: usize) -> Self {
        SimpleTableDataModel {
            col_count,
            data: vec![],
        }
    }

    pub fn set_columns(&mut self, cols: usize) {
        self.col_count = cols;
    }

    pub fn push(&mut self, cell: String) {
        self.data.push(cell);
    }

    pub fn append(&mut self, mut other: Vec<String>) {
        self.data.append(&mut other);
    }
}

impl Default for SimpleTableDataModel {
    fn default() -> Self {
        SimpleTableDataModel::new(0)
    }
}

impl TableDataModel for SimpleTableDataModel {
    fn is_empty(&self) -> bool {
        self.col_count == 0
    }

    fn col_count(&self) -> usize {
        self.col_count
    }

    fn row_count(&self) -> usize {
        if self.data.is_empty() || self.col_count == 0 {
            return 0;
        }

        let extra = if self.data.len() % self.col_count() > 0 {
            1
        } else {
            0
        };
        self.data.len() / self.col_count as usize + extra
    }

    fn draw_cell(&self, ui: &Ui, col: usize, row: usize) {
        if col >= self.col_count() {
            return;
        }
        if let Some(datum) = self.data.get(row * self.col_count + col) {
            ui.text(datum.as_str());
        }
    }
}

#[derive(Debug)]
// TODO: Can the underlying table's vectors get changed after adding?
pub struct CompoundTableDataModel<T: TableDataModel + Debug> {
    max_cols: usize,
    total_rows: usize,
    tables: Vec<Box<T>>,
}

impl<T: TableDataModel + Debug> CompoundTableDataModel<T> {
    fn new() -> Self {
        CompoundTableDataModel {
            max_cols: 0,
            total_rows: 0,
            tables: vec![],
        }
    }

    fn push(&mut self, table: T) {
        let col_count = table.col_count();
        if col_count > self.max_cols {
            self.max_cols = col_count
        };
        self.total_rows += table.row_count();
        self.tables.push(Box::new(table));
    }
}

impl<T: TableDataModel + Debug> TableDataModel for CompoundTableDataModel<T> {
    fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }

    fn col_count(&self) -> usize {
        self.max_cols
    }

    fn row_count(&self) -> usize {
        self.total_rows
    }

    fn draw_cell(&self, ui: &Ui, col: usize, row: usize) {
        if self.tables.is_empty() {
            return;
        }
        if col >= self.col_count() {
            return;
        }

        let mut actual_row = row;
        for table in &self.tables {
            if table.row_count() <= row {
                actual_row -= table.row_count();
            } else {
                table.draw_cell(ui, col, actual_row);
            }
        }
    }
}

pub fn draw_table(ui: &Ui, title: &str, data_model: &TableDataModel) {
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

    let col_count = data_model.col_count();
    let row_count = data_model.row_count();
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
            ui.with_font(FONT_IDX_NORMAL, || {
                for row in 0..row_count {
                    for col in 0..col_count {
                        data_model.draw_cell(ui, col, row);
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
