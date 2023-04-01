//! The `GameState` struct and its methods.
//! The `GameState` struct represents the state of a Tic Tac Toe game.
//! It contains the current state of the game board, and the mark of the player who goes first

use crate::logic::{validators, Cell, GameMove, Grid, Mark};

/// Represents the state of a Tic Tac Toe game.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct GameState {
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
    pub fn new(grid: Grid, starting_mark: Option<Mark>) -> Result<Self, String> {
        let game_state = {
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
        };
        validators::validate_game_state(&game_state)?;
        Ok(game_state)
    }

    /// Returns the current `Mark` of the player whose turn it is to make a move.
    ///
    /// The current mark is determined by checking the number of `naught`s and `cross`s in the `grid`.
    /// If the number of `naught`s is equal to the number of `cross`s, the `starting_mark` is returned.
    /// Otherwise, the other `Mark` is returned.
    pub fn current_mark(&self) -> Mark {
        if self.grid.naught_count() == self.grid.cross_count() {
            return self.starting_mark;
        }
        self.starting_mark.other()
    }

    /// Returns the winner's `Mark`, if there is one, otherwise returns `None`.
    pub fn winner_mark(&self) -> Option<Mark> {
        for mark in [Mark::Cross, Mark::Naught] {
            // Check rows
            for i in (0..Grid::SIZE).step_by(Grid::WIDTH) {
                let idx = i..i + Grid::WIDTH;
                let row = &self.grid.cells()[idx];
                if row.iter().all(|cell| cell.is_occupied_by(mark)) {
                    return Some(mark);
                }
            }

            // Check columns
            for i in 0..Grid::WIDTH {
                let column = (i..Grid::SIZE).step_by(Grid::WIDTH);

                if column
                    .clone()
                    .all(|j| self.grid.cells()[j].is_occupied_by(mark))
                {
                    return Some(mark);
                }
            }

            // Check diagonals
            let diagonal1 = (0..Grid::SIZE).step_by(Grid::WIDTH + 1);
            if diagonal1
                .clone()
                .all(|i| self.grid.cells()[i].is_occupied_by(mark))
            {
                return Some(mark);
            }

            let diagonal2 = (Grid::WIDTH - 1..Grid::SIZE - 1).step_by(Grid::WIDTH - 1);
            if diagonal2
                .clone()
                .all(|i| self.grid.cells()[i].is_occupied_by(mark))
            {
                return Some(mark);
            }
        }
        None
    }

    /// Returns the indexes of the winning cells for the given `Mark`.
    pub fn winning_indexes(&self) -> Option<Vec<usize>> {
        for mark in [Mark::Cross, Mark::Naught] {
            let mut winning_indexes: Vec<usize> = Vec::new();

            for i in (0..Grid::SIZE).step_by(Grid::WIDTH) {
                // Check rows
                let row = &self.grid.cells()[i..i + Grid::WIDTH];
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
                    .all(|j| self.grid.cells()[j].is_occupied_by(mark))
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
                .all(|i| self.grid.cells()[i].is_occupied_by(mark))
            {
                winning_indexes.extend(winning_indexes_temp);
                return Some(winning_indexes);
            }

            let diagonal2 = (Grid::WIDTH - 1..Grid::SIZE - 1).step_by(Grid::WIDTH - 1);
            let winning_indexes_temp = diagonal2.clone();
            if diagonal2
                .clone()
                .all(|i| self.grid.cells()[i].is_occupied_by(mark))
            {
                winning_indexes.extend(winning_indexes_temp);
                return Some(winning_indexes);
            }
        }
        None
    }

    /// Returns `true` if the game has not started, `false` otherwise.
    pub fn game_not_started(&self) -> bool {
        self.grid.empty_count() == 9
    }

    /// Returns `true` if the game is over, `false` otherwise.
    pub fn game_over(&self) -> bool {
        self.winner_mark().is_some() || self.tie()
    }

    /// Returns `true` if the game is over in a tie, `false` otherwise.
    pub fn tie(&self) -> bool {
        self.grid.empty_count() == 0 && self.winner_mark().is_none()
    }

    /// Makes a move to the specified cell index and returns a new `GameMove` object.
    ///
    /// # Arguments
    ///
    /// * `cell_index` - The index of the cell where the move should be made.
    ///
    /// # Returns
    ///
    /// A `Result` that contains either the `GameMove` object if the move is valid or an error message if the move is invalid.
    pub(crate) fn make_move_to(&self, cell_index: usize) -> Result<GameMove, String> {
        if self.grid.cells()[cell_index].is_occupied() {
            return Err(String::from("Cell is not empty"));
        }

        let mut new_cells = [Cell::new_empty(); Grid::SIZE];
        new_cells[..cell_index].copy_from_slice(&self.grid.cells()[..cell_index]);
        new_cells[cell_index] = Cell::new_marked(self.current_mark());
        new_cells[cell_index + 1..].copy_from_slice(&self.grid.cells()[cell_index + 1..]);

        let new_grid = Grid::new(Some(new_cells));
        let new_state = match GameState::new(new_grid, Some(self.starting_mark)) {
            Ok(state) => state,
            Err(error) => return Err(error),
        };

        Ok(GameMove::new(
            self.current_mark(),
            cell_index,
            *self,
            new_state,
        ))
    }

    /// Returns a vector of all possible moves for the current state of the game.
    ///
    /// If the game is already over, returns an empty vector.
    ///
    /// # Returns
    ///
    /// A vector of `GameMove` structs, each representing a possible move in the game.
    pub(crate) fn possible_moves(&self) -> Vec<GameMove> {
        let mut moves: Vec<GameMove> = Vec::new();
        if !self.game_over() {
            self.grid.cells().iter().enumerate().for_each(|(i, cell)| {
                if cell.is_vacant() {
                    if let Ok(possible_move) = self.make_move_to(i) {
                        moves.push(possible_move);
                    }
                }
            })
        }
        moves
    }

    pub(crate) fn grid(&self) -> &Grid {
        &self.grid
    }

    pub(crate) fn starting_mark(&self) -> &Mark {
        &self.starting_mark
    }

    pub(crate) fn score(&self, maximized_player: Mark) -> Result<i32, String> {
        if self.game_over() {
            if self.tie() {
                return Ok(0);
            } else if self.winner_mark() == Some(maximized_player) {
                return Ok(1);
            } else {
                return Ok(-1);
            }
        }
        Err(String::from("Game is not over"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_starting_mark() {
        let grid = Grid::new(None);
        let game_state = GameState::new(grid, Some(Mark::Naught)).unwrap();
        assert_eq!(game_state.starting_mark(), &Mark::Naught);
    }

    #[test]
    fn test_new_without_starting_mark() {
        let grid = Grid::new(None);
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.starting_mark(), &Mark::Cross);
    }

    #[test]
    fn test_current_mark_none() {
        let game_state = GameState::new(Grid::new(None), None).unwrap();
        assert_eq!(game_state.current_mark(), Mark::Cross);
    }

    #[test]
    fn test_current_mark_starting_mark_cross() {
        let game_state = GameState::new(Grid::new(None), Some(Mark::Cross)).unwrap();
        assert_eq!(game_state.current_mark(), Mark::Cross);
    }

    #[test]
    fn test_current_mark_starting_mark_naught() {
        let game_state = GameState::new(Grid::new(None), Some(Mark::Naught)).unwrap();
        assert_eq!(game_state.current_mark(), Mark::Naught);
    }

    // #[test]
    // fn test_current_mark_starting_mark_cross_one_move() {
    //     let mut game_state = GameState::new(Grid::new(None), Some(Mark::Cross)).unwrap();
    //     game_state.grid.cells()[0] = Cell::new_marked(Mark::Cross);
    //     assert_eq!(game_state.current_mark(), Mark::Naught);
    // }

    // #[test]
    // fn test_current_mark_starting_mark_naught_one_move() {
    //     let mut game_state = GameState::new(Grid::new(None), Some(Mark::Naught)).unwrap();
    //     game_state.grid.cells()[0] = Cell::new_marked(Mark::Naught);
    //     assert_eq!(game_state.current_mark(), Mark::Cross);
    // }

    #[test]
    fn test_winner_mark_none() {
        let grid = Grid::new(None);
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.winner_mark(), None);
    }

    #[test]
    fn test_winner_mark_row() {
        let mut cells = [Cell::new_empty(); Grid::SIZE];
        cells[0] = Cell::new_marked(Mark::Cross);
        cells[1] = Cell::new_marked(Mark::Cross);
        cells[2] = Cell::new_marked(Mark::Cross);

        cells[3] = Cell::new_marked(Mark::Naught);
        cells[4] = Cell::new_marked(Mark::Naught);
        let grid = Grid::new(Some(cells));
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.winner_mark(), Some(Mark::Cross));
    }

    #[test]
    fn test_winner_mark_column() {
        let mut cells = [Cell::new_empty(); Grid::SIZE];
        cells[0] = Cell::new_marked(Mark::Cross);
        cells[3] = Cell::new_marked(Mark::Cross);
        cells[6] = Cell::new_marked(Mark::Cross);

        cells[7] = Cell::new_marked(Mark::Naught);
        cells[8] = Cell::new_marked(Mark::Naught);
        let grid = Grid::new(Some(cells));
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.winner_mark(), Some(Mark::Cross));
    }

    #[test]
    fn test_winner_mark_diagonal() {
        let mut cells = [Cell::new_empty(); Grid::SIZE];
        cells[0] = Cell::new_marked(Mark::Cross);
        cells[4] = Cell::new_marked(Mark::Cross);
        cells[8] = Cell::new_marked(Mark::Cross);

        cells[7] = Cell::new_marked(Mark::Naught);
        cells[6] = Cell::new_marked(Mark::Naught);
        let grid = Grid::new(Some(cells));
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.winner_mark(), Some(Mark::Cross));
    }

    #[test]
    fn test_winner_mark_false() {
        let mut cells = [Cell::new_empty(); Grid::SIZE];
        cells[1] = Cell::new_marked(Mark::Cross);
        cells[4] = Cell::new_marked(Mark::Cross);
        cells[8] = Cell::new_marked(Mark::Cross);

        cells[7] = Cell::new_marked(Mark::Naught);
        cells[6] = Cell::new_marked(Mark::Naught);
        let grid = Grid::new(Some(cells));
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.winner_mark(), None);
    }

    #[test]
    fn test_winner_cells_none() {
        let grid = Grid::new(None);
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.winning_indexes(), None);
    }

    #[test]
    fn test_winner_cells_row() {
        let mut cells = [Cell::new_empty(); Grid::SIZE];
        cells[0] = Cell::new_marked(Mark::Cross);
        cells[1] = Cell::new_marked(Mark::Cross);
        cells[2] = Cell::new_marked(Mark::Cross);

        cells[3] = Cell::new_marked(Mark::Naught);
        cells[4] = Cell::new_marked(Mark::Naught);
        let grid = Grid::new(Some(cells));
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.winning_indexes(), Some(vec![0, 1, 2]));
    }

    #[test]
    fn test_winner_cells_column() {
        let mut cells = [Cell::new_empty(); Grid::SIZE];
        cells[0] = Cell::new_marked(Mark::Cross);
        cells[3] = Cell::new_marked(Mark::Cross);
        cells[6] = Cell::new_marked(Mark::Cross);

        cells[7] = Cell::new_marked(Mark::Naught);
        cells[8] = Cell::new_marked(Mark::Naught);
        let grid = Grid::new(Some(cells));
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.winning_indexes(), Some(vec![0, 3, 6]));
    }

    #[test]
    fn test_winner_cells_diagonal() {
        let mut cells = [Cell::new_empty(); Grid::SIZE];
        cells[0] = Cell::new_marked(Mark::Cross);
        cells[4] = Cell::new_marked(Mark::Cross);
        cells[8] = Cell::new_marked(Mark::Cross);

        cells[7] = Cell::new_marked(Mark::Naught);
        cells[6] = Cell::new_marked(Mark::Naught);
        let grid = Grid::new(Some(cells));
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.winning_indexes(), Some(vec![0, 4, 8]));
    }

    #[test]
    fn test_winner_cells_false() {
        let mut cells = [Cell::new_empty(); Grid::SIZE];
        cells[1] = Cell::new_marked(Mark::Cross);
        cells[4] = Cell::new_marked(Mark::Cross);
        cells[8] = Cell::new_marked(Mark::Cross);

        cells[7] = Cell::new_marked(Mark::Naught);
        cells[6] = Cell::new_marked(Mark::Naught);
        let grid = Grid::new(Some(cells));
        let game_state = GameState::new(grid, None).unwrap();
        assert_eq!(game_state.winning_indexes(), None);
    }

    #[test]
    fn test_game_not_started() {
        let empty_game = GameState::new(Grid::new(None), None).unwrap();
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
        )
        .unwrap();

        assert!(empty_game.game_not_started());
        assert!(!non_empty_game.game_not_started());
    }

    #[test]
    fn test_game_over() {
        let empty_game = GameState::new(Grid::new(None), None).unwrap();
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
        )
        .unwrap();
        let cross_wins_game = GameState::new(
            Grid::new(Some([
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Naught),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
            ])),
            None,
        )
        .unwrap();
        let naught_wins_game = GameState::new(
            Grid::new(Some([
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Cross),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Cross),
                Cell::new_empty(),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Naught),
            ])),
            None,
        )
        .unwrap();

        assert!(!empty_game.game_over());
        assert!(tie_game.game_over());
        assert!(cross_wins_game.game_over());
        assert!(naught_wins_game.game_over());
    }

    #[test]
    fn test_tie() {
        let empty_game = GameState::new(Grid::new(None), None).unwrap();
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
        )
        .unwrap();

        assert!(!empty_game.tie());
        assert!(non_empty_game.tie());
    }

    #[test]
    fn test_make_move_to_empty_cell() {
        let game = GameState::new(Grid::new(None), Some(Mark::Cross)).unwrap();
        let result = game.make_move_to(0);
        assert!(result.is_ok());
        let mv = result.unwrap();
        assert_eq!(mv.mark(), &Mark::Cross);
        assert_eq!(mv.cell_index(), 0);
        assert_eq!(mv.before_state(), &game);
        assert_eq!(mv.after_state().starting_mark(), game.starting_mark());
        assert_eq!(
            mv.after_state().grid().cells()[0],
            Cell::new_marked(Mark::Cross)
        );
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
        let game = GameState::new(Grid::new(Some(cells)), Some(Mark::Cross)).unwrap();
        let result = game.make_move_to(0);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, "Cell is not empty");
    }

    #[test]
    fn test_possible_moves_empty_game() {
        let game = GameState::new(Grid::new(None), None).unwrap();
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
        let game = GameState::new(grid, Some(Mark::Cross)).unwrap();
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
            Cell::new_marked(Mark::Cross),
        ]));
        let game = GameState::new(grid, Some(Mark::Cross)).unwrap();
        let moves = game.possible_moves();
        assert!(moves.is_empty());
    }
}
