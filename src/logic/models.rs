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
    pub fn new_marked(mark: Mark) -> Self {
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
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
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

    /// Returns the current `Mark` of the player whose turn it is to make a move.
    ///
    /// The current mark is determined by checking the number of `naught`s and `cross`s in the `grid`.
    /// If the number of `naught`s is equal to the number of `cross`s, the `starting_mark` is returned.
    /// Otherwise, the other `Mark` is returned.
    fn current_mark(&self) -> Mark {
        if self.grid.naught_count() == self.grid.cross_count() {
            return self.starting_mark;
        }
        self.starting_mark.other()
    }

    /// Returns the winner's `Mark`, if there is one, otherwise returns `None`.
    fn winner_mark(&self) -> Option<Mark> {
        for mark in [Mark::Cross, Mark::Naught] {
            // Check rows
            for i in (0..Grid::SIZE).step_by(Grid::WIDTH) {
                let idx = i..i + Grid::WIDTH;
                let row = &self.grid.cells[idx];
                if row.iter().all(|cell| cell.is_occupied_by(mark)) {
                    return Some(mark);
                }
            }

            // Check columns
            for i in 0..Grid::WIDTH {
                let column = (i..Grid::SIZE).step_by(Grid::WIDTH);

                if column
                    .clone()
                    .all(|j| self.grid.cells[j].is_occupied_by(mark))
                {
                    return Some(mark);
                }
            }

            // Check diagonals
            let diagonal1 = (0..Grid::SIZE).step_by(Grid::WIDTH + 1);
            if diagonal1
                .clone()
                .all(|i| self.grid.cells[i].is_occupied_by(mark))
            {
                return Some(mark);
            }

            let diagonal2 = (Grid::WIDTH - 1..Grid::SIZE - 1).step_by(Grid::WIDTH - 1);
            if diagonal2
                .clone()
                .all(|i| self.grid.cells[i].is_occupied_by(mark))
            {
                return Some(mark);
            }
        }
        None
    }

    /// Returns the indexes of the winning cells for the given `Mark`.
    fn winning_indexes(&self) -> Option<Vec<usize>> {
        for mark in [Mark::Cross, Mark::Naught] {
            let mut winning_indexes: Vec<usize> = Vec::new();

            for i in (0..Grid::SIZE).step_by(Grid::WIDTH) {
                // Check rows
                let row = &self.grid.cells[i..i + Grid::WIDTH];
                if row.iter().all(|cell| cell.is_occupied_by(mark)) {
                    winning_indexes.extend(i..i + Grid::WIDTH);
                    return Some(winning_indexes);
                }
            }

            for i in 0..Grid::WIDTH {
                // Check columns
                let column = (i..Grid::SIZE).step_by(Grid::WIDTH);

                if column
                    .clone()
                    .all(|j| self.grid.cells[j].is_occupied_by(mark))
                {
                    winning_indexes.extend(column);
                    return Some(winning_indexes);
                }
            }

            // Check diagonals
            let diagonal1 = (0..Grid::SIZE).step_by(Grid::WIDTH + 1);
            let winning_indexes_temp = diagonal1.clone();
            if diagonal1
                .clone()
                .all(|i| self.grid.cells[i].is_occupied_by(mark))
            {
                winning_indexes.extend(winning_indexes_temp);
                return Some(winning_indexes);
            }

            let diagonal2 = (Grid::WIDTH - 1..Grid::SIZE - 1).step_by(Grid::WIDTH - 1);
            let winning_indexes_temp = diagonal2.clone();
            if diagonal2
                .clone()
                .all(|i| self.grid.cells[i].is_occupied_by(mark))
            {
                winning_indexes.extend(winning_indexes_temp);
                return Some(winning_indexes);
            }
        }
        None
    }

    /// Returns `true` if the game has not started, `false` otherwise.
    fn game_not_started(&self) -> bool {
        self.grid.empty_count() == 9
    }

    /// Returns `true` if the game is over, `false` otherwise.
    fn game_over(&self) -> bool {
        self.winner_mark().is_some() || self.tie()
    }

    /// Returns `true` if the game is over in a tie, `false` otherwise.
    fn tie(&self) -> bool {
        self.grid.empty_count() == 0 && self.winner_mark().is_none()
    }

    /// Makes a move to the specified cell index and returns a new `Move` object.
    ///
    /// # Arguments
    ///
    /// * `cell_index` - The index of the cell where the move should be made.
    ///
    /// # Returns
    ///
    /// A `Result` that contains either the `Move` object if the move is valid or an error message if the move is invalid.
    fn make_move_to(&self, cell_index: usize) -> Result<Move, String> {
        if self.grid.cells[cell_index].is_occupied() {
            return Err(String::from("Cell is not empty"));
        }

        let mut new_cells = [Cell::new_empty(); Grid::SIZE];
        new_cells[..cell_index].copy_from_slice(&self.grid.cells[..cell_index]);
        new_cells[cell_index] = Cell::new_marked(self.current_mark());
        new_cells[cell_index + 1..].copy_from_slice(&self.grid.cells[cell_index + 1..]);

        let new_grid = Grid::new(Some(new_cells));
        let new_state = GameState::new(new_grid, Some(self.starting_mark));

        Ok(Move {
            mark: self.current_mark(),
            cell_index,
            before_state: *self,
            after_state: new_state,
        })
    }

    /// Returns a vector of all possible moves for the current state of the game.
    ///
    /// If the game is already over, returns an empty vector.
    ///
    /// # Returns
    ///
    /// A vector of `Move` structs, each representing a possible move in the game.
    fn possible_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        if !self.game_over() {
            self.grid.cells.iter().enumerate().for_each(|(i, cell)| {
                if cell.is_vacant() {
                    if let Ok(possible_move) = self.make_move_to(i) {
                        moves.push(possible_move);
                    }
                }
            })
        }
        moves
    }
}

/// Represents a move in a tic-tac-toe game.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Move {
    mark: Mark,
    cell_index: usize,
    before_state: GameState,
    after_state: GameState,
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
            let cell = Cell::new_marked(Mark::Cross);
            assert!(cell.is_occupied_by(Mark::Cross));
        }

        #[test]
        fn test_new_occupied_naught() {
            let cell = Cell::new_marked(Mark::Naught);
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
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_marked(Mark::Naught),
                ],
            };
            assert_eq!(grid.empty_count(), 5);
        }

        #[test]
        fn test_naught_count() {
            let grid = Grid {
                cells: [
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_marked(Mark::Naught),
                ],
            };
            assert_eq!(grid.naught_count(), 2);
        }

        #[test]
        fn test_cross_count() {
            let grid = Grid {
                cells: [
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_marked(Mark::Naught),
                ],
            };
            assert_eq!(grid.cross_count(), 4);
        }

        #[test]
        fn test_new_with_cells() {
            let cells = [
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Naught),
                Cell::new_empty(),
                Cell::new_marked(Mark::Cross),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_marked(Mark::Naught),
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

        #[test]
        fn test_current_mark_none() {
            let game_state = GameState::new(Grid::new(None), None);
            assert_eq!(game_state.current_mark(), Mark::Cross);
        }

        #[test]
        fn test_current_mark_starting_mark_cross() {
            let game_state = GameState::new(Grid::new(None), Some(Mark::Cross));
            assert_eq!(game_state.current_mark(), Mark::Cross);
        }

        #[test]
        fn test_current_mark_starting_mark_naught() {
            let game_state = GameState::new(Grid::new(None), Some(Mark::Naught));
            assert_eq!(game_state.current_mark(), Mark::Naught);
        }

        #[test]
        fn test_current_mark_starting_mark_cross_one_move() {
            let mut game_state = GameState::new(Grid::new(None), Some(Mark::Cross));
            game_state.grid.cells[0] = Cell::new_marked(Mark::Cross);
            assert_eq!(game_state.current_mark(), Mark::Naught);
        }

        #[test]
        fn test_current_mark_starting_mark_naught_one_move() {
            let mut game_state = GameState::new(Grid::new(None), Some(Mark::Naught));
            game_state.grid.cells[0] = Cell::new_marked(Mark::Naught);
            assert_eq!(game_state.current_mark(), Mark::Cross);
        }

        #[test]
        fn test_winner_mark_none() {
            let grid = Grid::new(None);
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.winner_mark(), None);
        }

        #[test]
        fn test_winner_mark_row() {
            let mut cells = [Cell::new_empty(); Grid::SIZE];
            cells[0] = Cell::new_marked(Mark::Cross);
            cells[1] = Cell::new_marked(Mark::Cross);
            cells[2] = Cell::new_marked(Mark::Cross);
            let grid = Grid::new(Some(cells));
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.winner_mark(), Some(Mark::Cross));
        }

        #[test]
        fn test_winner_mark_column() {
            let mut cells = [Cell::new_empty(); Grid::SIZE];
            cells[0] = Cell::new_marked(Mark::Cross);
            cells[3] = Cell::new_marked(Mark::Cross);
            cells[6] = Cell::new_marked(Mark::Cross);
            let grid = Grid::new(Some(cells));
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.winner_mark(), Some(Mark::Cross));
        }

        #[test]
        fn test_winner_mark_diagonal() {
            let mut cells = [Cell::new_empty(); Grid::SIZE];
            cells[0] = Cell::new_marked(Mark::Cross);
            cells[4] = Cell::new_marked(Mark::Cross);
            cells[8] = Cell::new_marked(Mark::Cross);
            let grid = Grid::new(Some(cells));
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.winner_mark(), Some(Mark::Cross));
        }

        #[test]
        fn test_winner_mark_false() {
            let mut cells = [Cell::new_empty(); Grid::SIZE];
            cells[1] = Cell::new_marked(Mark::Cross);
            cells[4] = Cell::new_marked(Mark::Cross);
            cells[8] = Cell::new_marked(Mark::Cross);
            let grid = Grid::new(Some(cells));
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.winner_mark(), None);
        }

        #[test]
        fn test_winner_cells_none() {
            let grid = Grid::new(None);
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.winning_indexes(), None);
        }

        #[test]
        fn test_winner_cells_row() {
            let mut cells = [Cell::new_empty(); Grid::SIZE];
            cells[0] = Cell::new_marked(Mark::Cross);
            cells[1] = Cell::new_marked(Mark::Cross);
            cells[2] = Cell::new_marked(Mark::Cross);
            let grid = Grid::new(Some(cells));
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.winning_indexes(), Some(vec![0, 1, 2]));
        }

        #[test]
        fn test_winner_cells_column() {
            let mut cells = [Cell::new_empty(); Grid::SIZE];
            cells[0] = Cell::new_marked(Mark::Cross);
            cells[3] = Cell::new_marked(Mark::Cross);
            cells[6] = Cell::new_marked(Mark::Cross);
            let grid = Grid::new(Some(cells));
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.winning_indexes(), Some(vec![0, 3, 6]));
        }

        #[test]
        fn test_winner_cells_diagonal() {
            let mut cells = [Cell::new_empty(); Grid::SIZE];
            cells[0] = Cell::new_marked(Mark::Cross);
            cells[4] = Cell::new_marked(Mark::Cross);
            cells[8] = Cell::new_marked(Mark::Cross);
            let grid = Grid::new(Some(cells));
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.winning_indexes(), Some(vec![0, 4, 8]));
        }

        #[test]
        fn test_winner_cells_false() {
            let mut cells = [Cell::new_empty(); Grid::SIZE];
            cells[1] = Cell::new_marked(Mark::Cross);
            cells[4] = Cell::new_marked(Mark::Cross);
            cells[8] = Cell::new_marked(Mark::Cross);
            let grid = Grid::new(Some(cells));
            let game_state = GameState::new(grid, None);
            assert_eq!(game_state.winning_indexes(), None);
        }

        #[test]
        fn test_game_not_started() {
            let empty_game = GameState::new(Grid::new(None), None);
            let non_empty_game = GameState::new(
                Grid::new(Some([
                    Cell::new_marked(Mark::Cross),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                ])),
                None,
            );

            assert!(empty_game.game_not_started());
            assert!(!non_empty_game.game_not_started());
        }

        #[test]
        fn test_game_over() {
            let empty_game = GameState::new(Grid::new(None), None);
            let tie_game = GameState::new(
                Grid::new(Some([
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Naught),
                ])),
                None,
            );
            let cross_wins_game = GameState::new(
                Grid::new(Some([
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                ])),
                None,
            );
            let naught_wins_game = GameState::new(
                Grid::new(Some([
                    Cell::new_marked(Mark::Naught),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_empty(),
                    Cell::new_marked(Mark::Naught),
                ])),
                None,
            );

            assert!(!empty_game.game_over());
            assert!(tie_game.game_over());
            assert!(cross_wins_game.game_over());
            assert!(naught_wins_game.game_over());
        }

        #[test]
        fn test_tie() {
            let empty_game = GameState::new(Grid::new(None), None);
            let non_empty_game = GameState::new(
                Grid::new(Some([
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Naught),
                    Cell::new_marked(Mark::Cross),
                    Cell::new_marked(Mark::Naught),
                ])),
                None,
            );

            assert!(!empty_game.tie());
            assert!(non_empty_game.tie());
        }

        #[test]
        fn test_make_move_to_empty_cell() {
            let game = GameState::new(Grid::new(None), Some(Mark::Cross));
            let result = game.make_move_to(0);
            assert!(result.is_ok());
            let mv = result.unwrap();
            assert_eq!(mv.mark, Mark::Cross);
            assert_eq!(mv.cell_index, 0);
            assert_eq!(mv.before_state, game);
            assert_eq!(mv.after_state.starting_mark, game.starting_mark);
            assert_eq!(mv.after_state.grid.cells[0], Cell::new_marked(Mark::Cross));
        }

        #[test]
        fn test_make_move_to_occupied_cell() {
            let cells = [
                Cell::new_marked(Mark::Cross),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_marked(Mark::Naught),
                Cell::new_empty(),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Naught),
                Cell::new_empty(),
                Cell::new_empty(),
            ];
            let game = GameState::new(Grid::new(Some(cells)), Some(Mark::Cross));
            let result = game.make_move_to(0);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(err, "Cell is not empty");
        }

        #[test]
        fn test_possible_moves_empty_game() {
            let game = GameState::new(Grid::new(None), None);
            let moves = game.possible_moves();
            assert_eq!(moves.len(), 9);
        }

        #[test]
        fn test_possible_moves_game_in_progress() {
            let grid = Grid::new(Some([
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Naught),
                Cell::new_empty(),
            ]));
            let game = GameState::new(grid, Some(Mark::Cross));
            let moves = game.possible_moves();
            assert_eq!(moves.len(), 5);
        }

        #[test]
        fn test_possible_moves_game_over() {
            let grid = Grid::new(Some([
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Naught),
            ]));
            let game = GameState::new(grid, Some(Mark::Cross));
            let moves = game.possible_moves();
            assert!(moves.is_empty());
        }
    }
}
