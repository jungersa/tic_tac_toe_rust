use crate::logic::models::{Mark, GameState, Move};
use crate::game::players::Player;




pub struct MinimaxPlayer {
    mark: Mark,
}

impl MinimaxPlayer {
    pub fn new(mark: Mark) -> Self {
        MinimaxPlayer { mark }
    }
}

impl Player for MinimaxPlayer {
    fn get_move(&self, game_state: &GameState) -> Option<Move> {
        find_best_move(game_state)
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}

fn find_best_move(game_state: &GameState) -> Option<Move> {
    let maximized_player = game_state.current_mark();
    game_state.possible_moves().into_iter().max_by_key(|move_| {
        minimax(move_, maximized_player, false)
    })
}

fn minimax(move_: &Move, maximized_player: Mark, choose_highest_score: bool) -> i32 {
    if move_.after_state().game_over() {
        return move_.after_state().score(maximized_player).unwrap();
    };
    let scores = move_.after_state().possible_moves().into_iter().map(|move_| {
        minimax(&move_, maximized_player, !choose_highest_score)
    });
    // println!("{:#?}", scores);
    return if choose_highest_score {
        scores.max().unwrap()
    } else {
        scores.min().unwrap()
    }
}