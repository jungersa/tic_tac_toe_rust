use super::{Mark, GameState};

/// Represents a move in a tic-tac-toe game.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct GameMove {
    mark: Mark,
    cell_index: usize,
    before_state: GameState,
    after_state: GameState,
}

impl GameMove {
    pub fn new(mark: Mark, cell_index: usize, before_state: GameState, after_state: GameState) -> Self {
        GameMove {
            mark,
            cell_index,
            before_state,
            after_state,
        }
    }

    /// Returns the mark of the move.
    pub fn mark(&self) -> &Mark {
        &self.mark
    }

    /// Returns the index of the cell where the move was made.
    pub fn cell_index(&self) -> usize {
        self.cell_index
    }

    /// Returns the after_state of the move.
    pub fn before_state(&self) -> &GameState {
        &self.before_state
    }

    /// Returns the after_state of the move.
    pub fn after_state(&self) -> &GameState {
        &self.after_state
    }
}
