use super::*;

/// Failed to place a number in a cell
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PlaceError {
    AlreadyInCell,
    AlreadyInRow,
    AlreadyInCol,
    AlreadyInSeg,
}

impl Board {
    pub fn placement_error(&self, pos: impl Position, cell: Cell) -> Option<PlaceError> {
        let index = pos.index();

        if self.cells[index] == cell {
            return Some(PlaceError::AlreadyInCell);
        }
        if self.row(index).contains(&cell) {
            return Some(PlaceError::AlreadyInRow);
        }
        if self.col(index).contains(&cell) {
            return Some(PlaceError::AlreadyInCol);
        }
        if self.segment(index).contains(&cell) {
            return Some(PlaceError::AlreadyInSeg);
        }
        None
    }

    pub fn place(&mut self, pos: impl Position, cell: Cell) -> Result<(), PlaceError> {
        match self.placement_error(pos, cell) {
            Some(e) => Err(e),
            None => {
                self.cells[pos.index()] = cell;
                Ok(())
            }
        }
    }
    pub fn place_at(&mut self, pos: impl Position, cell: Cell) -> Result<(), PlaceError> {
        self.place(pos, cell)
    }
}
