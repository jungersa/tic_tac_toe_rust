/// Represents a mark on the board in a Tic Tac Toe game.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Mark {
    /// The mark representing a cross, which is denoted by the string "X".
    Cross,
    /// The mark representing a naught, which is denoted by the string "O".
    Naught,
}

impl Mark {
    /// Returns a new instance of the enum with the opposite variant.
    fn other(&self) -> Self {
        match self {
            Mark::Cross => Mark::Naught,
            Mark::Naught => Mark::Cross,
        }
    }
}

/// Represents a single cell on the Tic Tac Toe game board.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
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
/// Represents the game board grid.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Grid {
    cells: [Cell; Grid::SIZE],
}

impl Grid {
    const WIDTH: usize = 3;
    const SIZE: usize = Grid::WIDTH * Grid::WIDTH;

    /// Creates a new `Grid` with the given list of `Cell`.
    ///
    /// If no list of `Cell` is provided, the default is a list of empty cells.
    ///
    /// # Arguments
    ///
    /// * `cells` - The list of cells size of Grid::SIZE.
    ///
    fn new(cells: Option<[Cell; Grid::SIZE]>) -> Self {
        if let Some(cell) = cells {
            Self { cells: cell }
        } else {
            Self {
                cells: [Cell::new_empty(); Grid::SIZE],
            }
        }
    }

    /// Returns the number of empty cells in the grid.
    fn empty_count(&self) -> usize {
        self.cells.iter().filter(|&cell| cell.is_vacant()).count()
    }

    /// Returns the number of cells which are naught in the grid.
    fn naught_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|&cell| cell.is_occupied_by(Mark::Naught))
            .count()
    }

    /// Returns the number of cells which are cross in the grid.
    fn cross_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|&cell| cell.is_occupied_by(Mark::Cross))
            .count()
    }
}

/// Represents the state of a Tic Tac Toe game.
struct GameState {
    /// The current state of the game board.
    grid: Grid,
    /// The mark of the player who goes first.
    starting_mark: Mark,
}

impl GameState {
    /// Creates a new `GameState` with the given `Grid` and starting `Mark`.
    ///
    /// If no starting `Mark` is provided, the default starting `Mark` is Mark::Cross.
    ///
    /// # Arguments
    ///
    /// * `grid` - The game board.
    /// * `starting_mark` - The mark of the player who goes first.
    ///
    fn new(grid: Grid, starting_mark: Option<Mark>) -> Self {
        if let Some(mark) = starting_mark {
            Self {
                grid,
                starting_mark: mark,
            }
        } else {
            Self {
                grid,
                starting_mark: Mark::Cross,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mark {
        use super::*;

        #[test]
        fn test_other_naught() {
            let cross = Mark::Cross;
            let naught = cross.other();
            assert_eq!(naught, Mark::Naught);
        }

        #[test]
        fn test_other_cross() {
            let naught = Mark::Naught;
            let cross = naught.other();
            assert_eq!(cross, Mark::Cross);
        }
    }

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

    mod grid {
        use super::*;
        #[test]
        fn test_empty_count_full() {
            let grid = Grid {
                cells: [Cell::new_empty(); Grid::SIZE],
            };
            assert_eq!(grid.empty_count(), Grid::SIZE);
        }

        #[test]
        fn test_empty_count() {
            let grid = Grid {
                cells: [
                    Cell::new_used(Mark::Cross),
                    Cell::new_used(Mark::Cross),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_used(Mark::Naught),
                    Cell::new_used(Mark::Naught),
                ],
            };
            assert_eq!(grid.empty_count(), 5);
        }

        #[test]
        fn test_naught_count() {
            let grid = Grid {
                cells: [
                    Cell::new_used(Mark::Cross),
                    Cell::new_used(Mark::Cross),
                    Cell::new_used(Mark::Cross),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_used(Mark::Naught),
                    Cell::new_used(Mark::Naught),
                ],
            };
            assert_eq!(grid.naught_count(), 2);
        }

        #[test]
        fn test_cross_count() {
            let mut grid = Grid {
                cells: [
                    Cell::new_used(Mark::Cross),
                    Cell::new_used(Mark::Cross),
                    Cell::new_used(Mark::Cross),
                    Cell::new_used(Mark::Cross),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_used(Mark::Naught),
                    Cell::new_used(Mark::Naught),
                    Cell::new_used(Mark::Naught),
                ],
            };
            assert_eq!(grid.cross_count(), 4);
        }

        #[test]
        fn test_new_with_cells() {
            let cells = [
                Cell::new_used(Mark::Cross),
                Cell::new_used(Mark::Naught),
                Cell::new_empty(),
                Cell::new_used(Mark::Cross),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_used(Mark::Naught),
            ];
            let grid = Grid::new(Some(cells));

            assert_eq!(grid.cells.len(), 9);
            assert_eq!(grid.cells[0].mark, Some(Mark::Cross));
            assert_eq!(grid.cells[1].mark, Some(Mark::Naught));
            assert_eq!(grid.cells[2].mark, None);
            assert_eq!(grid.cells[3].mark, Some(Mark::Cross));
            assert_eq!(grid.cells[4].mark, None);
            assert_eq!(grid.cells[5].mark, None);
            assert_eq!(grid.cells[6].mark, None);
            assert_eq!(grid.cells[7].mark, None);
            assert_eq!(grid.cells[8].mark, Some(Mark::Naught));
        }

        #[test]
        fn test_new_without_cells() {
            let grid = Grid::new(None);

            assert_eq!(grid.cells.len(), 9);
            for cell in grid.cells.iter() {
                assert!(cell.is_vacant());
            }
        }
    }

    mod gamestate {
        use super::*;

        #[test]
        fn test_new_with_starting_mark() {
            let grid = Grid::new(None);
            let game_state = GameState::new(grid, Some(Mark::Naught));
            assert_eq!(game_state.starting_mark, Mark::Naught);
        }

        #[test]
        fn test_new_without_starting_mark() {
            let grid = Grid::new(None);
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.starting_mark, Mark::Cross);
        }
    }
}
