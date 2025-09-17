#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod draw;
mod generate;
mod test;
mod theme;
mod ui;

use generate::Strategy;
use macroquad::prelude::*;

fn app() -> Conf {
    Conf {
        window_title: "Sudoru".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: true,
        sample_count: 4,
        high_dpi: true,

        ..Default::default()
    }
}

#[macroquad::main(app)]
async fn main() {
    let gen_strategy = Strategy::TryRandomSparse;
    let mut board = generate::generate_board(gen_strategy);

    let mut ui = ui::UI::new();

    println!("{board}");

    loop {
        ui.update();

        clear_background(ui.theme().bg);
        ui.draw_cells(&board);
        ui.draw_borders(&board);
        ui.draw_squares(&board);
        ui.handle_input(&mut board);

        next_frame().await;
    }
}
