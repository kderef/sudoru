use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::draw::DrawCanvas;
use crate::game::Game;
use crate::theme::Theme;

impl<'a> Game<'a> {
    pub fn draw_selected(&self, canvas: &mut WindowCanvas) {
        let Theme { selected_bg, .. } = self.theme;

        let cell_size = self.board_rect.width() as f32 / 9.;

        if let Some(pos) = self.selected_square {
            let pos = pos as u32;
            canvas.set_draw_color(selected_bg);

            let cell_w = self.board_rect.width() / self.board.width() as u32;
            let cell_h = self.board_rect.height() / self.board.height() as u32;

            let col = pos % self.board.width() as u32;
            let row = pos / self.board.width() as u32;

            let x = self.board_rect.x + (col * cell_w) as i32;
            let y = self.board_rect.y + (row * cell_h) as i32;

            let cell = Rect::new(x, y, cell_w, cell_h);
            canvas.fill_rect(cell).unwrap();
        }
    }

    pub fn draw_borders(&self, canvas: &mut WindowCanvas) {
        let Theme {
            border_color,
            border_thick,
            ..
        } = self.theme;

        canvas.set_draw_color(border_color);
        canvas.draw_rect_lines(self.board_rect, border_thick);
    }

    pub fn draw_cell_numbers(&self, canvas: &mut WindowCanvas) {
        let Theme { cell_fg, .. } = self.theme;

        let texture_creator = canvas.texture_creator();
        let surface = self
            .font
            .render("9")
            .blended(cell_fg) //
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let cell_size = self.board_rect.width() / 9;

        let start_x = self.board_rect.x;
        let start_y = self.board_rect.y;

        let char_size = cell_size / 2;
        let mut cell = Rect::new(start_x, start_y, cell_size, cell_size);

        for row in 0..self.board.height() {
            for col in 0..self.board.width() {
                let bounds = Rect::new(cell.x + char_size as i32 / 2, cell.y, char_size, cell_size);
                canvas.copy(&texture, None, Some(bounds));

                cell.x += cell.w;
            }
            cell.x = start_x;
            cell.y += cell.h;
        }
    }

    pub fn draw_segment_lines(&self, canvas: &mut WindowCanvas) {
        let Theme {
            square_color,
            square_thick,
            ..
        } = self.theme;

        let segments = self.board.segments() as u32;
        let segment_size = self.board_rect.width() / segments;

        canvas.set_draw_color(square_color);

        let left = self.board_rect.x;
        let right = self.board_rect.right();
        let top = self.board_rect.top();
        let bottom = self.board_rect.bottom();

        for row in 1..self.board.segments() {
            let y = self.board_rect.y + (row as u32 * segment_size) as i32;

            canvas.draw_hline(left, right, y, square_thick);
        }
        for col in 1..self.board.segments() {
            let x = self.board_rect.x + (col as u32 * segment_size) as i32;

            canvas.draw_vline(top, bottom, x, square_thick);
        }
    }

    pub fn draw_cell_lines(&self, canvas: &mut WindowCanvas) {
        let Theme {
            cell_border,
            cell_thick,
            ..
        } = self.theme;

        let cell_size = self.board_rect.width() as i32 / 9;

        canvas.set_draw_color(cell_border);

        // horizontal
        let l = self.board_rect.x;
        let r = self.board_rect.right();

        for row in 1..(self.board.height() as i32) {
            if row % 3 == 0 {
                continue;
            }

            let y = row * cell_size + self.board_rect.y;

            canvas.draw_hline(l, r, y, cell_thick);
        }
        // vertical
        let top = self.board_rect.top();
        let bottom = self.board_rect.bottom();

        for col in 1..(self.board.width() as i32) {
            if col % 3 == 0 {
                continue;
            }

            let x = col * cell_size + self.board_rect.x;

            canvas.draw_vline(top, bottom, x, cell_thick);
        }
    }
    /*
    fn a(&self, canvas: &mut WindowCanvas) {
        let cell_size = self.board_rect.width() / 9;
        let size = self.board_rect.width() / 3;

        for row in 0..3 {
            for col in 0..3 {
                canvas.set_draw_color(square_color);

                // draw section
                let x = self.board_rect.x + (size as i32 * col);
                let y = self.board_rect.y + (size as i32 * row);

                let section = Rect::new(x, y, size, size);

                canvas.draw_rect_lines(section, square_thick);

                // draw cells
                canvas.set_draw_color(cell_border);
                for cell_row in 0..3 {
                    for cell_col in 0..3 {
                        let cell = Rect::new(
                            x + (cell_size * cell_col) as i32,
                            y + (cell_size * cell_row) as i32,
                            cell_size,
                            cell_size,
                        );

                        canvas.draw_rect_lines(cell, cell_thick);
                    }
                }
            }
        }
    }
    */
}
