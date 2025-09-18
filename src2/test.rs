#[cfg(test)]
mod tests {
    use crate::{Board, board::PlaceError};

    #[test]
    fn board_row() {
        let mut board = Board::new();

        for i in 1..9 {
            board.cells[i] = Some(i as u8);
        }

        let row = board.row(0);

        assert_eq!(
            row,
            &[
                None,
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
            ]
        );
    }

    #[test]
    fn board_col() {
        let mut board = Board::new();

        for i in 1..9 {
            board.set(1, i, Some(i as u8));
        }

        let col = board.col(1);

        assert_eq!(
            &col,
            &[
                None,
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
            ]
        );
    }

    #[test]
    fn board_insert() {
        let mut board = Board::new();
        let value = Some(1);

        board.cells[0] = value;

        let result = board.place_at(2, 0, value);
        assert_eq!(result, Err(PlaceError::AlreadyInRow));

        let result = board.place_at(0, 2, value);
        assert_eq!(result, Err(PlaceError::AlreadyInCol));

        let result = board.place_at(1, 1, value);
        assert_eq!(result, Err(PlaceError::AlreadyInSeg));
    }

    #[test]
    fn board_get() {
        let mut board = Board::new();
        let value = Some(2);

        board.cells[10] = value;

        assert_eq!(board.get(1, 1), Some(&value));

        let value = Some(3);
        board.set(3, 3, value);

        assert_eq!(board.get(3, 3), Some(&value));
    }
}
