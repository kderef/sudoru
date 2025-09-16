use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub bg: Color,
    pub padding: f32,

    pub border_color: Color,
    pub border_thick: f32,

    pub square_color: Color,
    pub square_thick: f32,

    pub cell_border: Color,
    pub cell_fg: Color,
    pub cell_highlight: Color,
    pub cell_thick: f32,

    pub selected_bg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        LIGHT
    }
}

const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
}
const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1. }
}

pub static LIGHT: Theme = Theme {
    bg: rgb(0.9, 0.9, 0.9),
    padding: 0.05,

    border_color: rgb(0.2, 0.2, 0.2),
    border_thick: 4.0,

    square_color: rgb(0.2, 0.2, 0.2),
    square_thick: 2.0,

    cell_border: rgb(0.5, 0.5, 0.5),
    cell_fg: rgb(0., 0., 0.),
    cell_highlight: rgb(0.9, 0., 0.),
    cell_thick: 1.,

    selected_bg: rgba(0., 0., 0., 0.15),
};

pub static DARK: Theme = Theme {
    bg: BLACK,
    padding: 0.05,

    border_color: rgb(0.7, 0.2, 0.2),
    border_thick: 6.0,

    square_color: rgb(0.5, 0.2, 0.2),
    square_thick: 4.0,

    cell_border: rgb(0.5, 0.2, 0.2),
    cell_fg: rgb(0.9, 0.9, 0.9),
    cell_highlight: rgba(0.9, 0., 0., 1.0),
    cell_thick: 1.,

    selected_bg: rgb(0.1, 0.2, 0.4),
};
