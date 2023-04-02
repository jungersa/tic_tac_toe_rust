use clap::Parser;
use tic_tac_toe_rust::game::engine::TicTacToe;

mod cli;
use cli::{parse_cli, Cli};

fn main() {
    let cli = Cli::parse();

    let game_config = parse_cli(cli);

    let game = TicTacToe::new(
        game_config.player1.as_ref(),
        game_config.player2.as_ref(),
        game_config.renderer.as_ref(),
        None,
    );

    match game {
        Ok(game) => game.play(Some(game_config.starting_mark)),
        Err(err) => panic!("{}", err),
    }
}
