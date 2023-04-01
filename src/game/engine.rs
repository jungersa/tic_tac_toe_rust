//!    The TicTacToe struct represents a game of Tic Tac Toe that can be played by two players
//!    and rendered with a renderer.

use crate::logic::{GameState, Grid, Mark};

use super::players::Player;
use super::renderers::Renderer;

type ErrorHandler = dyn Fn(String);

/// TicTacToe game struct.
pub struct TicTacToe<'a> {
    player1: &'a dyn Player,
    player2: &'a dyn Player,
    renderer: &'a dyn Renderer,
    error_handler: Option<Box<ErrorHandler>>,
}

impl<'a> TicTacToe<'a> {
    /// Creates a new TicTacToe instance with two players, a renderer, and an optional error handler.
    /// Returns a Result containing the TicTacToe instance or an error message.
    ///
    /// # Arguments
    ///
    /// * player1 - The first player.
    /// * player2 - The second player.
    /// * renderer - The renderer used to display the game.
    /// * error_handler - An optional error handler function.
    pub fn new(
        player1: &'a dyn Player,
        player2: &'a dyn Player,
        renderer: &'a dyn Renderer,
        error_handler: Option<Box<ErrorHandler>>,
    ) -> Result<Self, String> {
        if player1.get_mark() == player2.get_mark() {
            return Err("Cannot play against each other".to_string());
        }

        Ok(TicTacToe {
            player1,
            player2,
            renderer,
            error_handler,
        })
    }

    /// Plays a game of Tic Tac Toe using the current `TicTacToe` instance.
    ///
    /// # Arguments
    ///
    /// * `starting_mark` - An optional starting mark for the game. If `None`, the starting mark is `Mark::Cross`.
    pub fn play(&self, starting_mark: Option<Mark>) {
        let mut game_state = GameState::new(Grid::new(None), starting_mark).unwrap();

        loop {
            self.renderer.render(&game_state);

            if game_state.game_over() {
                break;
            }

            let current_player = self.get_current_player(&game_state);

            match current_player.make_move(&game_state) {
                Ok(new_game_state) => game_state = new_game_state.to_owned(),
                Err(err) => {
                    if let Some(error_handler) = self.error_handler.as_ref() {
                        error_handler(err);
                    }
                }
            }
        }
    }

    /// Get the current player based on the current mark in the game state.
    ///
    /// # Arguments
    ///
    /// * `game_state` - The current game state.
    fn get_current_player(&self, game_state: &GameState) -> &'a dyn Player {
        if game_state.current_mark() == self.player1.get_mark() {
            self.player1
        } else {
            self.player2
        }
    }
}
