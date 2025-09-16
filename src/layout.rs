use macroquad::prelude::*;

use crate::draw;
use crate::ui::UI;
use crate::{board::Board, theme::Theme};

#[inline]
pub fn screen_size() -> (f32, f32) {
    (screen_width(), screen_height())
}

impl Board {
    pub fn draw(&self, ui: &UI) {
        ui.draw_cells(self);
        ui.draw_borders(self);
        ui.draw_squares(self);
    }
}
