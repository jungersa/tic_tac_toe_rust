//! The player used in the cli

use std::io;

use clap::builder::NonEmptyStringValueParser;

use crate::{
    game::players::Player,
    logic::{GameMove, GameState, Mark},
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
    /// Get the move from the player
    /// Using the standard input
    ///
    /// # Arguments
    ///
    /// * game_state - The curent `GameState` of the game
    fn get_move(&self, game_state: &GameState) -> Option<GameMove> {
        while !game_state.game_over() {
            let mut input_string = String::new();

            println!("{}'s move: ", self.mark);

            io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read input.");

            match coord_to_index(input_string.trim()) {
                Some(input) => {
                    if (0..9).contains(&input) {
                        if let Ok(next_move) = game_state.make_move_to(input) {
                            return Some(next_move);
                        };
                        println!("That cell is already occupied.");
                    } else {
                        println!("Invalid input. Try again.");
                    }
                }
                None => {
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


fn coord_to_index(coord: &str) -> Option<usize> {
    let chars: Vec<char> = coord.chars().collect();
    if chars.len() != 2 {
        return None;
    }
    let row = match chars[0] {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        '1' => 0,
        '2' => 1,
        '3' => 2,
        _ => return None,
    };
    let col = match chars[1] {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => return None,
    };
    Some(row * 3 + col)
}