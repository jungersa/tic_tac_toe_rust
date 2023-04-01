use tic_tac_toe_rust::game::engine::TicTacToe;
use tic_tac_toe_rust::game::players::minimax::MinimaxPlayer;
use tic_tac_toe_rust::game::{Player, Renderer};
use tic_tac_toe_rust::logic::Mark;

use tic_tac_toe_rust::frontend::console::players::{ConsolePlayer, DumbPlayer};
use tic_tac_toe_rust::frontend::console::renderers::ConsoleRenderer;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "Tic Tac Toe")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short='1', long, value_enum, default_value_t = PlayerType::Human)]
    player1: PlayerType,
    #[arg(short='2', long, value_enum, default_value_t = PlayerType::Human)]
    player2: PlayerType,
    #[arg(short, long, value_enum, default_value_t = StartingMark::Cross)]
    starting_mark: StartingMark,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum PlayerType {
    Human,
    ComputerMinimax,
    ComputerRandom,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum StartingMark {
    Cross,
    Naught,
}

struct GameConfig {
    player1: Box<dyn Player>,
    player2: Box<dyn Player>,
    renderer: Box<dyn Renderer>,
    starting_mark: Mark,
}

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

fn parse_cli(cli: Cli) -> GameConfig {
    let player1;

    if let PlayerType::Human = cli.player1 {
        player1 = Box::new(ConsolePlayer::new(Mark::Cross)) as Box<dyn Player>;
    } else if let PlayerType::ComputerMinimax = cli.player1 {
        player1 = Box::new(MinimaxPlayer::new(Mark::Cross)) as Box<dyn Player>;
    } else {
        player1 = Box::new(DumbPlayer::new(Mark::Cross)) as Box<dyn Player>;
    }

    let player2;

    if let PlayerType::Human = cli.player2 {
        player2 = Box::new(ConsolePlayer::new(Mark::Naught)) as Box<dyn Player>;
    } else if let PlayerType::ComputerMinimax = cli.player2 {
        player2 = Box::new(MinimaxPlayer::new(Mark::Naught)) as Box<dyn Player>;
    } else {
        player2 = Box::new(DumbPlayer::new(Mark::Naught)) as Box<dyn Player>;
    }

    let starting_mark = if let StartingMark::Cross = cli.starting_mark {
        Mark::Cross
    } else {
        Mark::Naught
    };

    let renderer = Box::new(ConsoleRenderer {}) as Box<dyn Renderer>;

    GameConfig {
        player1,
        player2,
        renderer,
        starting_mark,
    }
}
