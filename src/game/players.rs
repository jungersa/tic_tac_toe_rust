use crate::logic::models::{GameState, Mark, Move};
pub trait Player {
    fn make_move(&self, game_state: &GameState) -> Result<GameState, String> {
        if self.get_mark() == game_state.current_mark() {
            if let Some(next_move) = self.get_move(game_state) {
                return Ok(*next_move.after_state());
            }
            return Err("No more possible moves".to_string());
        }
        Err("It's the other player's turn".to_string())
    }
    fn get_mark(&self) -> Mark;
    fn get_move(&self, game_state: &GameState) -> Option<Move>;
}
