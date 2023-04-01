use std::io;

use crate::{
    game::players::Player,
    logic::models::{GameMove, GameState, Mark},
};

pub struct ConsolePlayer {
    mark: Mark,
}

impl ConsolePlayer {
    pub fn new(mark: Mark) -> Self {
        ConsolePlayer { mark }
    }
}

impl Player for ConsolePlayer {
    fn get_move(&self, game_state: &GameState) -> Option<GameMove> {
        while !game_state.game_over() {
            let mut input_string = String::new();

            println!("{}'s move: ", self.mark);

            io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read input.");

            match input_string.trim().parse() {
                Ok(input) => {
                    if (0..9).contains(&input) {
                        if let Ok(next_move) = game_state.make_move_to(input) {
                            return Some(next_move);
                        };
                        println!("That cell is already occupied.");
                    } else {
                        println!("Invalid input. Try again.");
                    }
                }
                Err(_) => {
                    println!("Invalid input. Try again.");
                }
            }
        }
        None
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}

pub struct DumbPlayer {
    mark: Mark,
}

impl DumbPlayer {
    pub fn new(mark: Mark) -> Self {
        DumbPlayer { mark }
    }
}

impl Player for DumbPlayer {
    fn get_move(&self, game_state: &GameState) -> Option<GameMove> {
        let moves = game_state.possible_moves();
        if moves.is_empty() {
            return None;
        }
        Some(moves[0])
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}
