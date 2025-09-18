mod index;
mod placement;

pub use index::Position;
pub use placement::PlaceError;

use std::fmt::Display;

pub type Cell = Option<u8>;

/// indexed by a number of 0..9 - 1
/// example: CELL_STR[2 - 1] == "2"
pub static CELL_STR: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

pub const SEGMENTS: usize = 3;
pub const WIDTH: usize = 9;
pub const HEIGHT: usize = 9;
pub const SIZE: usize = WIDTH * HEIGHT;

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub cells: [Cell; SIZE],
}
impl Board {
    pub const fn new() -> Self {
        Self { cells: [None; _] }
    }
    pub const fn width(&self) -> usize {
        WIDTH
    }
    pub const fn height(&self) -> usize {
        HEIGHT
    }
    pub const fn size(&self) -> usize {
        self.width() * self.height()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width();
        let height = self.height();
        let size = self.size();

        for (i, row) in self.cells.chunks_exact(size).enumerate() {
            if i % height == 0 && i != 0 {
                writeln!(f, "------+-------+------")?;
            }

            for (j, cell) in row.iter().enumerate() {
                if j % width == 0 && j != 0 {
                    write!(f, "| ")?;
                }
                match cell {
                    Some(val) => write!(f, "{} ", val)?,
                    None => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
