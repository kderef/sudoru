#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod draw;
mod generate;
mod layout;
mod test;
mod theme;
mod ui;

use macroquad::prelude::*;

use crate::board::Board;

fn app() -> Conf {
    Conf {
        window_title: "Sudoru".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: true,
        sample_count: 4,

        ..Default::default()
    }
}

#[macroquad::main(app)]
async fn main() {
    let current_theme = theme::LIGHT;
    let mut board = generate::generate_board();

    let mut ui = ui::UI::new(current_theme);

    println!("{board}");

    loop {
        // update
        ui.update();

        clear_background(current_theme.bg);
        board.draw(&ui);

        if let Some((index, num)) = ui.insert_num() {
            if let Err(e) = board.place(index, Some(num)) {
                println!("{index} => {e:?}");
                ui.highlight(&board, index, Some(num), e);
            } else {
                ui.highlighted_cell = None;
            }
        }

        next_frame().await;
    }
}
