use rand::Rng;

use crate::board::Board;

pub fn generate_board() -> Board {
    let mut board = Board::new();
    let mut rng = rand::rng();

    // strategy: try to place a number or skip it if it fails.
    for row in 0..board.height() {
        for col in 0..board.width() {
            let num = rng.random_range(1..=9);
            let _ = board.place_at(col, row, Some(num));
        }
    }

    board
}
