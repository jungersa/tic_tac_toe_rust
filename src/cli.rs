use tic_tac_toe_rust::{
    frontend::console::{
        players::{ConsolePlayer, DumbPlayer},
        renderers::ConsoleRenderer,
    },
    game::{MinimaxPlayer, Player, Renderer},
    logic::Mark,
};

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "Tic Tac Toe")]
#[command(author, version, about, long_about = None)]
pub(super) struct Cli {
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

pub(super) struct GameConfig {
    pub(super) player1: Box<dyn Player>,
    pub(super) player2: Box<dyn Player>,
    pub(super) renderer: Box<dyn Renderer>,
    pub(super) starting_mark: Mark,
}

pub(super) fn parse_cli(cli: Cli) -> GameConfig {
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
