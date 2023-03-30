use crate::logic::models::GameState;

pub trait Renderer {
    fn render(&self, game_state: &GameState);
}