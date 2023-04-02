//! The game module contains the TicTacToe struct, which is the main entry point for the game.
//! And it contains the Player trait, which is used to define the behavior of a player.
//! And it contains the Renderer trait, which is used to define the behavior of a renderer.
//! And it contains the minimax module, which contains the MinimaxPlayer struct, which is a player that uses the minimax algorithm to make moves.

pub mod engine;
pub mod players;
pub mod renderers;

pub use engine::TicTacToe;
pub use players::minimax::MinimaxPlayer;
pub use players::random::DumbPlayer;
pub use players::Player;
pub use renderers::Renderer;
