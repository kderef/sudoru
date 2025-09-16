use macroquad::prelude::*;

use crate::draw;
use crate::ui::UI;
use crate::{board::Board, theme::Theme};

#[inline]
pub fn screen_size() -> (f32, f32) {
    (screen_width(), screen_height())
}

pub fn board(screen_width: f32, padding: f32) -> Rect {
    let w = screen_width - padding * 2.;

    Rect {
        x: padding,
        y: padding,
        w,
        h: w,
    }
}

impl Board {
    pub fn draw(&self, ui: &UI) {
        ui.draw_borders();
        ui.draw_squares(self);
    }
}
