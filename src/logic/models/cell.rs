use super::mark::Mark;

/// Represents a single cell on the Tic Tac Toe game board.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Cell {
    mark: Option<Mark>,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.mark {
            Some(mark) => mark.fmt(f),
            None => write!(f, " "),
        }
    }
}

impl Cell {
    /// Create a new empty cell.
    pub(crate) fn new_empty() -> Self {
        Self { mark: None }
    }

    /// Creates a new `Cell` with a mark.
    ///
    /// # Arguments
    ///
    /// * `mark` - The mark which will be in the cell
    ///
    pub(crate) fn new_marked(mark: Mark) -> Self {
        Cell { mark: Some(mark) }
    }

    /// Returns `true` if the cell is occupied by a mark, `false` otherwise.
    pub(super) fn is_occupied(&self) -> bool {
        self.mark.is_some()
    }

    /// Returns `true` if the cell is empty, `false` if it is occupied by a mark.
    pub(super) fn is_vacant(&self) -> bool {
        !self.is_occupied()
    }

    /// Returns `true` if the cell is occupied by the specified mark, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `mark` - The mark to check for in the cell.
    ///
    pub(super) fn is_occupied_by(&self, mark: Mark) -> bool {
        if let Some(m) = self.mark {
            m == mark
        } else {
            false
        }
    }
}

mod tests {
    use super::super::mark::Mark;
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
        let cell = Cell::new_marked(Mark::Cross);
        assert!(cell.is_occupied_by(Mark::Cross));
    }

    #[test]
    fn test_new_occupied_naught() {
        let cell = Cell::new_marked(Mark::Naught);
        assert!(cell.is_occupied_by(Mark::Naught));
    }
}
