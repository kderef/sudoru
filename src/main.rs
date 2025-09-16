#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod draw;
mod generate;
mod test;
mod theme;
mod ui;

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
    let mut board = generate::generate_board();

    let mut ui = ui::UI::new();

    println!("{board}");

    loop {
        // update
        ui.update();

        clear_background(ui.theme().bg);
        ui.draw_cells(&board);
        ui.draw_borders(&board);
        ui.draw_squares(&board);

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
