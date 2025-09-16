use macroquad::prelude::*;
use std::fmt::Display;

pub type Cell = Option<u8>;

pub static CELL_STR: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

pub const SEGMENTS: usize = 3;
pub const WIDTH: usize = 9;
pub const HEIGHT: usize = 9;
pub const SIZE: usize = WIDTH * HEIGHT;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PlaceError {
    AlreadyInCell,
    AlreadyInRow,
    AlreadyInCol,
    AlreadyInSeg,
}

pub fn nearest_multiple(n: usize, base: usize) -> usize {
    let remainder = n % base;
    if remainder * 2 < base {
        n.checked_sub(remainder).unwrap_or(0)
    } else {
        n.checked_sub(base + remainder).unwrap_or(0)
    }
}

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
        SIZE
    }

    pub const fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width()
    }
    pub fn cell_pos(&self, index: usize) -> (usize, usize) {
        (index % self.height(), index / self.width())
    }
    pub fn cell_pos_vec(&self, index: usize) -> Vec2 {
        let pos = self.cell_pos(index);
        vec2(pos.0 as f32, pos.1 as f32)
    }

    pub fn row(&self, index: usize) -> &[Cell] {
        let (_, y) = self.cell_pos(index);
        self.cells.chunks_exact(WIDTH).nth(y).unwrap()
    }
    pub fn col(&self, index: usize) -> [Cell; 9] {
        let (x, _) = self.cell_pos(index);
        let mut cells = [None; _];

        for (i, c) in self.cells.iter().skip(x).step_by(HEIGHT).enumerate() {
            cells[i] = *c;
        }
        cells
    }

    pub fn segment_pos(&self, x: usize, y: usize) -> (usize, usize) {
        (x - x % 3, y - y % 3)
    }

    pub fn segment(&self, index: usize) -> [Cell; 9] {
        let (x, y) = self.cell_pos(index);
        let (x, y) = self.segment_pos(x, y);

        let mut cells = [None; _];

        /*
        (0, 0), (1, 0), (2, 0),
        (1, 0), (1, 1), (1, 2),
        (2, 0), (2, 1), (2, 2)
        */

        for row in 0..3 {
            for col in 0..3 {
                let cell = *self.get(x + col, y + row).unwrap();
                cells[col + row * 3] = cell;
            }
        }

        cells
    }

    pub fn place(&mut self, index: usize, cell: Cell) -> Result<(), PlaceError> {
        if self.cells[index] == cell {
            return Err(PlaceError::AlreadyInCell);
        }

        if self.row(index).contains(&cell) {
            return Err(PlaceError::AlreadyInRow);
        }
        if self.col(index).contains(&cell) {
            return Err(PlaceError::AlreadyInCol);
        }
        if self.segment(index).contains(&cell) {
            return Err(PlaceError::AlreadyInSeg);
        }

        self.cells[index] = cell;
        Ok(())
    }
    pub fn place_at(&mut self, x: usize, y: usize, cell: Cell) -> Result<(), PlaceError> {
        self.place(y * self.width() + x, cell)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y * WIDTH + x)
    }
    pub fn square(&mut self, index: usize) -> &mut [Cell] {
        todo!()
    }
    pub fn set(&mut self, x: usize, y: usize, value: Cell) {
        self.cells[self.index(x, y)] = value;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = 3;
        let height = 3;
        let size = width * height;

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
