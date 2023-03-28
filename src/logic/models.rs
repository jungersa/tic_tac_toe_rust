/// Represents a mark on the board in a Tic Tac Toe game.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Mark {
    /// The mark representing a cross, which is denoted by the string "X".
    Cross,
    /// The mark representing a naught, which is denoted by the string "O".
    Naught,
}

/// Represents a single cell on the Tic Tac Toe game board.
#[derive(Clone, Copy, Eq, PartialEq)]
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
    /// Create a new empty cell.
    fn new_empty() -> Self {
        Self { mark: None }
    }

    /// Creates a new `Cell` with a mark.
    ///
    /// # Arguments
    ///
    /// * `mark` - The mark which will be in the cell
    ///
    pub fn new_used(mark: Mark) -> Self {
        Cell { mark: Some(mark) }
    }

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
            assert!(empty_cell.is_vacant());
        }

        #[test]
        fn test_is_vacant_occupied() {
            let occupied_cell = Cell {
                mark: Some(Mark::Cross),
            };
            assert!(!occupied_cell.is_vacant());
        }

        #[test]
        fn test_is_occupied_empty() {
            let empty_cell = Cell { mark: None };
            assert!(!empty_cell.is_occupied());
        }

        #[test]
        fn test_is_occupied_occupied() {
            let occupied_cell = Cell {
                mark: Some(Mark::Cross),
            };
            assert!(occupied_cell.is_occupied());
        }

        #[test]
        fn test_is_occupied_by_empty() {
            let empty_cell = Cell { mark: None };
            assert!(!empty_cell.is_occupied_by(Mark::Cross));
            assert!(!empty_cell.is_occupied_by(Mark::Naught));
        }

        #[test]
        fn test_is_occupied_by_cross() {
            let occupied_by_cross = Cell {
                mark: Some(Mark::Cross),
            };
            assert!(occupied_by_cross.is_occupied_by(Mark::Cross));
            assert!(!occupied_by_cross.is_occupied_by(Mark::Naught));
        }

        #[test]
        fn test_is_occupied_by_naught() {
            let occupied_by_naught = Cell {
                mark: Some(Mark::Naught),
            };
            assert!(!occupied_by_naught.is_occupied_by(Mark::Cross));
            assert!(occupied_by_naught.is_occupied_by(Mark::Naught));
        }

        #[test]
        fn test_new_empty() {
            let cell = Cell::new_empty();
            assert!(cell.is_vacant());
        }

        #[test]
        fn test_new_occupied_cross() {
            let cell = Cell::new_used(Mark::Cross);
            assert!(cell.is_occupied_by(Mark::Cross));
        }

        #[test]
        fn test_new_occupied_naught() {
            let cell = Cell::new_used(Mark::Naught);
            assert!(cell.is_occupied_by(Mark::Naught));
        }
    }
}
