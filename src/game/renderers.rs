//! Renderers for the game.
use crate::logic::GameState;

/// A trait for rendering the game.
/// A renderer has a single method, render, which takes a game state and renders it.
pub trait Renderer {
    fn render(&self, game_state: &GameState);
}
