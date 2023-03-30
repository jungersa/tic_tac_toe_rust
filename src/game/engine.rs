use crate::logic::models::{GameState, Grid, Mark};

use super::players::Player;
use super::renderers::Renderer;

type ErrorHandler = dyn Fn(String);

pub struct TicTacToe<'a> {
    player1: &'a dyn Player,
    player2: &'a dyn Player,
    renderer: &'a dyn Renderer,
    error_handler: Option<Box<ErrorHandler>>,
}

impl<'a> TicTacToe<'a> {
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

    fn get_current_player(&self, game_state: &GameState) -> &'a dyn Player {
        if game_state.current_mark() == self.player1.get_mark() {
            self.player1
        } else {
            self.player2
        }
    }
}
