use std::fmt::Display;

pub type Cell = Option<u8>;

/// indexed by a number of 0..9 - 1
/// example: CELL_STR[2 - 1] == "2"
pub static CELL_STR: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

pub const SEGMENTS: usize = 3;
pub const WIDTH: usize = 9;
pub const HEIGHT: usize = 9;
pub const SIZE: usize = WIDTH * HEIGHT;

/// Failed to place a number in a cell
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PlaceError {
    AlreadyInCell,
    AlreadyInRow,
    AlreadyInCol,
    AlreadyInSeg,
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
        self.width() * self.height()
    }

    #[inline]
    pub const fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width()
    }
    #[inline]
    pub fn cell_pos(&self, index: usize) -> (usize, usize) {
        (index % self.height(), index / self.width())
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

    /// round to the position of the nearest segment (1, 2) -> (0, 3)
    pub fn segment_pos(&self, x: usize, y: usize) -> (usize, usize) {
        (x - x % SEGMENTS, y - y % SEGMENTS)
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
        self.place(self.index(x, y), cell)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(self.index(x, y))
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
