use crate::board::CELL_STR;
use crate::board::{self, HEIGHT, WIDTH};
use crate::ui::UI;
use crate::{board::Board, layout, theme::Theme};
use macroquad::prelude::*;

impl UI {
    pub fn draw_borders(&self) {
        let Rect { x, y, w, h } = self.board_layout;
        draw_rectangle_lines(
            x, //
            y,
            w,
            h,
            self.theme.border_thick,
            self.theme.border_color,
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
            self.theme.cell_highlight
        } else {
            self.theme.cell_fg
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

        for y in 0..board.height() {
            for x in 0..board.width() {
                if let Some(Some(num)) = board.get(x, y) {
                    let highlight = Some(board.index(x, y)) == self.highlighted_cell;
                    self.draw_cell_num(cell, *num, highlight);
                }
                cell.x += cell.w;
            }
            cell.x = start_x;
            cell.y += cell.h;
        }
    }

    pub fn draw_squares(&self, board: &Board) {
        let Theme {
            square_thick,
            square_color: square,
            cell_thick,
            cell_bg: cell_color,
            ..
        } = self.theme;

        let size = self.board_layout.w / 3.;
        let cell_size = size / 3.;

        let selected_pos = self.selected_cell.map(|pos| {
            // transform to point
            (pos % board::HEIGHT, pos / board::WIDTH)
        });

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

                        let actual_pos = (col * 3 + cell_col, row * 3 + cell_row);
                        if Some(actual_pos) == selected_pos {
                            draw_rectangle(cell.x, cell.y, cell.w, cell.h, self.theme.selected_bg);
                        }
                        // TODO: draw cell numbers
                    }
                }
            }
        }

        self.draw_cells(board);
    }
}
