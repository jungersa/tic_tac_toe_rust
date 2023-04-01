use tic_tac_toe_rust::game::engine::TicTacToe;
use tic_tac_toe_rust::game::players::minimax::MinimaxPlayer;
use tic_tac_toe_rust::logic::Mark;

use tic_tac_toe_rust::frontend::console::players::ConsolePlayer;
use tic_tac_toe_rust::frontend::console::renderers::ConsoleRenderer;

fn main() {
    let player1 = ConsolePlayer::new(Mark::Cross);
    let player2 = MinimaxPlayer::new(Mark::Naught);

    let renderer = ConsoleRenderer {};
    let game = TicTacToe::new(&player1, &player2, &renderer, None);

    match game {
        Ok(game) => game.play(None),
        Err(err) => eprint!("{}", err),
    }
}
