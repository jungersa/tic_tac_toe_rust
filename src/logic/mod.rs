//! This module contains the logic of the game.
//! It contains the models, which are the data structures used in the game.
//! And it contains the validators, which are the functions that validate the game state.

pub mod models;
mod validators;

pub use models::cell::Cell;
pub use models::game_move::GameMove;
pub use models::game_state::GameState;
pub use models::grid::Grid;
pub use models::mark::Mark;
