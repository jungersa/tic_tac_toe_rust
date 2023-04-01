//!   The Tic-Tac-Toe game.
//!   This crate provides the logic for the game, as well as a basic frontend for CLI.
//!   The game is played by two players, who take turns marking the spaces in a 3×3 grid.
//!   The player who succeeds in placing three of their marks in a horizontal, vertical, or diagonal row is the winner.
//!   The game can be played versus another human player or versus a computer player.
//!   The computer player can be configured to play randomly or to use the minimax algorithm.

pub mod frontend;
pub mod game;
pub mod logic;
