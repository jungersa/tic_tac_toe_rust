/// Represents a mark on the board in a Tic Tac Toe game.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Mark {
    /// The mark representing a cross, which is denoted by the string "X".
    Cross,
    /// The mark representing a naught, which is denoted by the string "O".
    Naught,
}

/// Represents a single cell on the Tic Tac Toe game board.
struct Cell {
    mark: Option<Mark>,
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Mark::Cross => write!(f, "X"),
            Mark::Naught => write!(f, "O"),
        }
    }
}

impl Cell {
    /// Returns `true` if the cell is occupied by a mark, `false` otherwise.
    fn is_occupied(&self) -> bool {
        self.mark.is_some()
    }

    /// Returns `true` if the cell is empty, `false` if it is occupied by a mark.
    fn is_vacant(&self) -> bool {
        !self.is_occupied()
    }

    /// Returns `true` if the cell is occupied by the specified mark, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `mark` - The mark to check for in the cell.
    ///
    fn is_occupied_by(&self, mark: Mark) -> bool {
        if let Some(m) = self.mark {
            m == mark
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod cell {
        use super::*;
        #[test]
        fn test_is_vacant_empty() {
            let empty_cell = Cell { mark: None };
            assert_eq!(empty_cell.is_vacant(), true);
        }

        #[test]
        fn test_is_vacant_occupied() {
            let occupied_cell = Cell {
                mark: Some(Mark::Cross),
            };
            assert_eq!(occupied_cell.is_vacant(), false);
        }

        #[test]
        fn test_is_occupied_empty() {
            let empty_cell = Cell { mark: None };
            assert_eq!(empty_cell.is_occupied(), false);
        }

        #[test]
        fn test_is_occupied_occupied() {
            let occupied_cell = Cell {
                mark: Some(Mark::Cross),
            };
            assert_eq!(occupied_cell.is_occupied(), true);
        }

        #[test]
        fn test_is_occupied_by_empty() {
            let empty_cell = Cell { mark: None };
            assert_eq!(empty_cell.is_occupied_by(Mark::Cross), false);
            assert_eq!(empty_cell.is_occupied_by(Mark::Naught), false);
        }

        #[test]
        fn test_is_occupied_by_cross() {
            let occupied_by_cross = Cell {
                mark: Some(Mark::Cross),
            };
            assert_eq!(occupied_by_cross.is_occupied_by(Mark::Cross), true);
            assert_eq!(occupied_by_cross.is_occupied_by(Mark::Naught), false);
        }

        #[test]
        fn test_is_occupied_by_naught() {
            let occupied_by_naught = Cell {
                mark: Some(Mark::Naught),
            };
            assert_eq!(occupied_by_naught.is_occupied_by(Mark::Cross), false);
            assert_eq!(occupied_by_naught.is_occupied_by(Mark::Naught), true);
        }
    }
}
