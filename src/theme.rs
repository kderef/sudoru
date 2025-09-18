use sdl2::pixels::Color;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub bg: Color,
    pub padding: f32,

    pub border_color: Color,
    pub border_thick: u32,

    pub square_color: Color,
    pub square_thick: u32,

    pub cell_border: Color,
    pub cell_fg: Color,
    pub cell_highlight: Color,
    pub cell_thick: u32,

    pub selected_bg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        LIGHT
    }
}
impl Default for &'static Theme {
    fn default() -> Self {
        &LIGHT
    }
}

const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}
const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b, a: 0xff }
}

pub static LIGHT: Theme = Theme {
    bg: rgb(200, 200, 200),
    padding: 0.05,

    border_color: rgb(50, 50, 50),
    border_thick: 8,

    square_color: rgb(50, 50, 50),
    square_thick: 4,

    cell_border: rgb(125, 125, 125),
    cell_fg: rgb(0, 0, 0),
    cell_highlight: rgb(230, 0, 0),
    cell_thick: 4,

    selected_bg: rgba(170, 170, 170, 50),
};

// pub static DARK: Theme = Theme {
//     bg: BLACK,
//     padding: 0.05,

//     border_color: rgb(0.7, 0.2, 0.2),
//     border_thick: 6.0,

//     square_color: rgb(0.5, 0.2, 0.2),
//     square_thick: 4.0,

//     cell_border: rgb(0.5, 0.2, 0.2),
//     cell_fg: rgb(0.9, 0.9, 0.9),
//     cell_highlight: rgba(0.9, 0., 0., 1.0),
//     cell_thick: 1.,

//     selected_bg: rgb(0.1, 0.2, 0.4),
// };
