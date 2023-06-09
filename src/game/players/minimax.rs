//! A player that uses the minimax algorithm to find the best move.
//! The minimax algorithm is a recursive algorithm that finds the best move for a player.
//! It works by recursively finding the best move for the maximized player and the best move for the minimized player.
//! The maximized player is the player whose turn it is.
//! The minimized player is the other player.
use crate::{
    game::players::Player,
    logic::{GameMove, GameState, Mark},
};

/// A player that uses the minimax algorithm to find the best move.
pub struct MinimaxPlayer {
    mark: Mark,
}

impl MinimaxPlayer {
    /// Creates a new MinimaxPlayer with the given mark.
    ///
    /// # Arguments
    ///
    /// * `mark` - The mark of the player.
    pub fn new(mark: Mark) -> Self {
        MinimaxPlayer { mark }
    }
}

impl Player for MinimaxPlayer {
    fn get_move(&self, game_state: &GameState) -> Option<GameMove> {
        find_best_move(game_state)
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}

/// Finds the best move for the maximized player.
///
/// # Arguments
///
/// * `game_state` - The game state to find the best move for.
fn find_best_move(game_state: &GameState) -> Option<GameMove> {
    let maximized_player = game_state.current_mark();
    let alpha = i32::MIN;
    let beta = i32::MAX;

    game_state
        .possible_moves()
        .into_iter()
        .max_by_key(|move_| minimax_with_pruning(move_, maximized_player, false, alpha, beta))
}

/// Finds the score of the given move.
/// The score is the score of the after_state of the move.
/// If the after_state is not a game over state, the score is the score of the best move for the other player.
/// The best move for the other player is the move with the highest score if the maximized player is the other player.
/// The best move for the other player is the move with the lowest score if the maximized player is the maximized player.
///
/// # Arguments
///
/// * `move_` - The move to find the score of.
/// * `maximized_player` - The maximized player.
/// * `choose_highest_score` - Whether to choose the highest score or the lowest score.
#[allow(dead_code)]
fn minimax(move_: &GameMove, maximized_player: Mark, choose_highest_score: bool) -> i32 {
    if move_.after_state().game_over() {
        return move_.after_state().score(maximized_player).unwrap();
    };
    let scores = move_
        .after_state()
        .possible_moves()
        .into_iter()
        .map(|move_| minimax(&move_, maximized_player, !choose_highest_score));
    if choose_highest_score {
        scores.max().unwrap()
    } else {
        scores.min().unwrap()
    }
}

/// Finds the score of the given move.
/// The score is the score of the after_state of the move.
/// If the after_state is not a game over state, the score is the score of the best move for the other player.
/// The best move for the other player is the move with the highest score if the maximized player is the other player.
/// The best move for the other player is the move with the lowest score if the maximized player is the maximized player.
/// Use alpha-beta pruning to speed up the algorithm.
///
/// # Arguments
///
/// * `move_` - The move to find the score of.
/// * `maximized_player` - The maximized player.
/// * `choose_highest_score` - Whether to choose the highest score or the lowest score.
/// * `alpha` - The alpha value.
/// * `beta` - The beta value.
fn minimax_with_pruning(
    move_: &GameMove,
    maximized_player: Mark,
    choose_highest_score: bool,
    alpha: i32,
    beta: i32,
) -> i32 {
    if move_.after_state().game_over() {
        return move_.after_state().score(maximized_player).unwrap();
    }

    let mut best_score = if choose_highest_score {
        i32::MIN
    } else {
        i32::MAX
    };

    let mut new_alpha = alpha;
    let mut new_beta = beta;

    for child_move in move_.after_state().possible_moves() {
        let score = minimax_with_pruning(
            &child_move,
            maximized_player,
            !choose_highest_score,
            new_alpha,
            new_beta,
        );

        if choose_highest_score {
            best_score = best_score.max(score);
            new_alpha = new_alpha.max(score);
        } else {
            best_score = best_score.min(score);
            new_beta = new_beta.min(score);
        }

        if beta <= alpha {
            break; // alpha-beta pruning
        }
    }

    best_score
}
