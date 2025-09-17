use crate::board::{CELL_STR, Position};
use crate::ui::UI;
use crate::{board::Board, theme::Theme};
use macroquad::prelude::*;

impl UI {
    pub fn draw_borders(&self, _board: &Board) {
        let Rect { x, y, w, h } = self.board_layout;
        draw_rectangle_lines(
            x, //
            y,
            w,
            h,
            self.theme().border_thick,
            self.theme().border_color,
        );
    }

    pub fn draw_cell_num(&self, cell: Rect, num: u8, highlight: bool) {
        let font = None;
        let font_size = self.font_size;

        let num_str = CELL_STR[num as usize - 1];

        let text_size = measure_text(num_str, font, font_size, 1.);
        let offset = vec2(text_size.width / 2., 0.);

        let text_pos = cell.center() - offset;
        let color = if highlight {
            self.theme().cell_highlight
        } else {
            self.theme().cell_fg
        };

        draw_text_ex(
            num_str,
            text_pos.x,
            text_pos.y,
            TextParams {
                font,
                font_size,
                color,
                ..Default::default()
            },
        );
    }
    pub fn draw_cells(&self, board: &Board) {
        let cell_size = self.board_layout.w / 9.;
        let start_x = self.board_layout.x;
        let start_y = self.board_layout.y;

        let mut cell = Rect::new(start_x, start_y, cell_size, cell_size);

        // Draw the selected cell
        if let Some(pos) = self.selected_cell {
            let (x, y) = (pos % board.height(), pos / board.width());
            let (x, y) = (x as f32 * cell.w + start_x, y as f32 * cell.h + start_y);

            draw_rectangle(x, y, cell.w, cell.h, self.theme().selected_bg);
        }

        for y in 0..board.height() {
            for x in 0..board.width() {
                let index = (x, y).index();
                if let Some(Some(num)) = board.get(index) {
                    let highlight = Some(index) == self.highlighted_cell;
                    self.draw_cell_num(cell, *num, highlight);
                }
                cell.x += cell.w;
            }
            cell.x = start_x;
            cell.y += cell.h;
        }
    }

    pub fn handle_input(&mut self, board: &mut Board) {
        if let Some((index, num)) = self.insert_num() {
            if let Err(e) = board.place(index, Some(num)) {
                println!("{index} => {e:?}");
                self.highlight(&board, index, Some(num), e);
            } else {
                self.highlighted_cell = None;
            }
        }
    }

    pub fn draw_squares(&self, _board: &Board) {
        let &Theme {
            square_thick,
            square_color: square,
            cell_thick,
            cell_border: cell_color,
            ..
        } = self.theme();

        let size = self.board_layout.w / 3.;
        let cell_size = size / 3.;

        for row in 0..3 {
            for col in 0..3 {
                // draw section
                let x = self.board_layout.x + (size * col as f32);
                let y = self.board_layout.y + (size * row as f32);
                draw_rectangle_lines(x, y, size, size, square_thick, square);

                // draw cells
                for cell_row in 0..3 {
                    for cell_col in 0..3 {
                        let cell = Rect::new(
                            x + cell_size * cell_col as f32,
                            y + cell_size * cell_row as f32,
                            cell_size,
                            cell_size,
                        );

                        draw_rectangle_lines(
                            cell.x, cell.y, cell.w, cell.h, cell_thick, cell_color,
                        );
                    }
                }
            }
        }
    }
}
