use crate::logic::{Mark, GameState, GameMove};

use super::Player;

/// A dumb player which take the first possible move to play
/// Need to be changed to random
pub struct DumbPlayer {
    mark: Mark,
}

impl DumbPlayer {
    pub fn new(mark: Mark) -> Self {
        DumbPlayer { mark }
    }
}

impl Player for DumbPlayer {
    fn get_move(&self, game_state: &GameState) -> Option<GameMove> {
        let moves = game_state.possible_moves();
        if moves.is_empty() {
            return None;
        }
        Some(moves[0])
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}
