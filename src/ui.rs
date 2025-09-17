use crate::{
    board::{Board, Cell, PlaceError},
    theme,
};
use macroquad::prelude::*;

use crate::board::Position;
use crate::theme::Theme;

pub fn screen_size() -> (f32, f32) {
    (screen_width(), screen_height())
}

pub fn board_layout(screen_size: Vec2, padding: f32) -> Rect {
    let w = screen_size.min_element() - padding * 2.;

    Rect {
        x: padding,
        y: padding,
        w,
        h: w,
    }
}

#[derive(Clone, Default)]
pub struct UI {
    pub themes: (&'static Theme, &'static Theme),

    pub board_layout: Rect,
    pub padding: f32,
    pub selected_cell: Option<usize>,
    pub screen_size: Vec2,
    pub font_size: u16,

    pub highlighted_cell: Option<usize>,
}

impl UI {
    pub fn new() -> Self {
        Self {
            themes: (&theme::LIGHT, &theme::DARK),
            ..Default::default()
        }
    }

    pub fn cycle_theme(&mut self) {
        self.themes = (self.themes.1, self.themes.0);
    }

    pub fn theme(&self) -> &Theme {
        self.themes.0
    }

    pub fn update(&mut self) {
        // toggle theme
        if is_key_pressed(KeyCode::T) {
            self.cycle_theme();
        }

        let new_screen_size = screen_size().into();
        if new_screen_size != self.screen_size {
            self.screen_size = new_screen_size;
            // recalc
            self.padding = self.theme().padding * self.screen_size.x;
            self.board_layout = board_layout(self.screen_size, self.padding);
            self.font_size = (0.05 * self.board_layout.w) as u16;
        }

        // cell selection
        if let Some(sel) = self.get_cell_clicked() {
            if self.selected_cell == Some(sel.index()) {
                self.selected_cell = None;
            } else {
                self.selected_cell = Some(sel.index());
            }
        }
    }
    pub fn insert_num(&self) -> Option<(usize, u8)> {
        if let Some(selected) = self.selected_cell {
            if let Some(num) = self.num_key_clicked() {
                return Some((selected, num));
            }
        }
        None
    }

    pub fn highlight(&mut self, board: &Board, index: impl Position, value: Cell, err: PlaceError) {
        // find offending cell
        let victim = value;
        let find = |group: &[Cell]| {
            group
                .iter()
                .enumerate()
                .find(|(_, c)| **c == victim)
                .map(|c| c.0)
                .unwrap()
        };
        let (x, y) = index.coords();

        let offender = match err {
            PlaceError::AlreadyInCell => index.index(),
            PlaceError::AlreadyInRow => (find(board.row(index)), y).index(),
            PlaceError::AlreadyInCol => (x, find(&board.col(index))).index(),
            PlaceError::AlreadyInSeg => {
                let (seg_x, seg_y) = index.segment().coords();
                let seg_index = find(&board.segment(index));
                (seg_x + seg_index % 3, seg_y + seg_index / 3).index()
            }
        };

        self.highlighted_cell = Some(offender);
    }

    pub fn get_cell_clicked(&self) -> Option<impl Position> {
        let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);

        if !mouse_clicked {
            return None;
        }

        let mouse_pos: Vec2 = mouse_position().into();
        if !self.board_layout.contains(mouse_pos) {
            return None;
        }

        let cell_size = self.board_layout.w / 9.;
        let pos = mouse_pos - self.board_layout.point();

        let index = (pos / cell_size).floor();
        let index = index.x as usize + index.y as usize * 9;

        Some(index)
    }

    pub fn num_key_clicked(&self) -> Option<u8> {
        let key = get_last_key_pressed()?;

        let num = match key {
            KeyCode::Key1 => 1,
            KeyCode::Key2 => 2,
            KeyCode::Key3 => 3,
            KeyCode::Key4 => 4,
            KeyCode::Key5 => 5,
            KeyCode::Key6 => 6,
            KeyCode::Key7 => 7,
            KeyCode::Key8 => 8,
            KeyCode::Key9 => 9,
            _ => return None,
        };

        Some(num)
    }
}
