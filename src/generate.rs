use rand::Rng;

use crate::board::{Board, Position};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Strategy {
    /// Try to insert and ignore if it fails
    TryRandom,
    /// Same as TryRandom, but it only inserts on random chance.
    TryRandomSparse,
}

pub fn generate_board(strategy: Strategy) -> Board {
    let mut board = Board::new();
    let mut rng = rand::rng();

    match strategy {
        Strategy::TryRandom => {
            for row in 0..board.height() {
                for col in 0..board.width() {
                    let num = rng.random_range(1..=9);
                    let _ = board.place_at((col, row), Some(num));
                }
            }
        }
        Strategy::TryRandomSparse => {
            for row in 0..board.height() {
                for col in 0..board.width() {
                    let num = rng.random_range(1..=9);
                    let should_place = rng.random_bool(1. / 3.);
                    let index = (col, row).index();

                    if should_place && board.placement_error(index, Some(num)).is_none() {
                        let _ = board.place(index, Some(num));
                    }
                }
            }
        }
    }

    board
}
