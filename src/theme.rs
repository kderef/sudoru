use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub bg: Color,
}

const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
}
const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1. }
}

pub static LIGHT: Theme = Theme {
    bg: rgb(0.9, 0.9, 0.9), //
};
