#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod draw;
mod generate;
mod test;
mod theme;
mod ui;

use std::{thread::sleep, time::Duration};

use generate::Strategy;
use macroquad::{miniquad::conf::Platform, prelude::*};
use ui::UI;

pub const SAMPLE_COUNT: i32 = 2;

fn app() -> Conf {
    Conf {
        window_title: "Sudoru".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: true,
        sample_count: SAMPLE_COUNT,
        high_dpi: true,
        platform: Platform {
            // swap_interval: Some(-1),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(app)]
async fn main() {
    let gen_strategy = Strategy::TryRandomSparse;
    let mut board = generate::generate_board(gen_strategy);

    let mut ui = UI::new();

    let min_frame_time = 1. / 30.;

    loop {
        let frame_time = get_frame_time();

        ui.update();

        ui.draw(&mut board);

        // sleep for CPU's sake
        if frame_time < min_frame_time {
            let sleep_time = (min_frame_time - frame_time) * 1000.;
            sleep(Duration::from_millis(sleep_time as u64));
        }
        next_frame().await;
    }
}
