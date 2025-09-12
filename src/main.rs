mod theme;

use macroquad::prelude::*;

fn app() -> Conf {
    Conf {
        window_title: "Sudoru".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: true,

        ..Default::default()
    }
}

#[macroquad::main(app)]
async fn main() {
    let current_theme = theme::LIGHT;

    loop {
        clear_background(current_theme.bg);

        next_frame().await;
    }
}
