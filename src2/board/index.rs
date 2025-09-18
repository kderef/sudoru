use macroquad::math::Vec2;

use super::*;

pub trait Position: Copy {
    /// index into board cells
    fn index(self) -> usize;

    /// (x, y)
    fn coords(self) -> (usize, usize);

    /// round to the position of the nearest segment (1, 2) -> (0, 3)
    fn segment(self) -> impl Position
    where
        Self: Sized,
    {
        let (x, y) = self.coords();
        (x - x % SEGMENTS, y - y % SEGMENTS)
    }
}

impl Position for usize {
    fn index(self) -> usize {
        self
    }
    fn coords(self) -> (usize, usize) {
        (self % WIDTH, self / HEIGHT)
    }
}
impl Position for (usize, usize) {
    fn index(self) -> usize {
        self.0 + self.1 * WIDTH
    }
    fn coords(self) -> (usize, usize) {
        self
    }
}
impl Position for Vec2 {
    fn coords(self) -> (usize, usize) {
        (self.x.floor() as usize, self.y.floor() as usize)
    }
    fn index(self) -> usize {
        let (x, y) = self.coords();
        x + y * WIDTH
    }
}

impl Board {
    pub fn row(&self, index: impl Position) -> &[Cell] {
        let (_, y) = index.coords();
        self.cells.chunks_exact(WIDTH).nth(y).unwrap()
    }
    pub fn col(&self, index: impl Position) -> [Cell; 9] {
        let (x, _) = index.coords();
        let mut cells = [None; _];

        for (i, c) in self.cells.iter().skip(x).step_by(HEIGHT).enumerate() {
            cells[i] = *c;
        }
        cells
    }

    pub fn segment(&self, pos: impl Position) -> [Cell; 9] {
        let (x, y) = pos.segment().coords();

        let mut cells = [None; _];

        /*
        (0, 0), (1, 0), (2, 0),
        (1, 0), (1, 1), (1, 2),
        (2, 0), (2, 1), (2, 2)
        */

        for row in 0..3 {
            for col in 0..3 {
                let cell = *self.get((x + col, y + row)).unwrap();
                cells[col + row * 3] = cell;
            }
        }

        cells
    }

    pub fn get(&self, pos: impl Position) -> Option<&Cell> {
        self.cells.get(pos.index())
    }
}
