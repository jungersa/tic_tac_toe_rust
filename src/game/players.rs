use crate::logic::models::{GameState, Move, Mark};
pub(crate) trait Player {
    fn make_move(&self, game_state: &GameState) -> Result<&GameState, String>;
    fn get_mark(&self) -> Mark;
    fn get_move(&self, game_state: &GameState) -> Option<Move>;
}
