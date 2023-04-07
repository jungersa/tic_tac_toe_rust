//! This module contains the Player trait and the implementations of the players.

use crate::logic::{errors::MoveError, GameMove, GameState, Mark};
pub mod minimax;
pub mod random;

/// The Player trait defines the behavior of a player.
/// A player trait has 3 methods:
/// - get_mark() returns the mark of the player
/// - get_move() returns the next move of the player
/// - make_move() returns the game state after the player has made a move
pub trait Player {
    fn make_move(&self, game_state: &GameState) -> Result<GameState, MoveError> {
        if self.get_mark() != game_state.current_mark() {
            return Err(MoveError::NotYourTurn(self.get_mark()));
        }
        if let Some(next_move) = self.get_move(game_state) {
            return Ok(*next_move.after_state());
        }
        Err(MoveError::NoPossibleMoves)
    }
    fn get_mark(&self) -> Mark;
    fn get_move(&self, game_state: &GameState) -> Option<GameMove>;
}
