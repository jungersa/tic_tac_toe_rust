use crate::logic::models::GameState;

pub(crate) trait Renderer {
    fn render(&self, game_state: &GameState);
}