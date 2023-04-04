//! The player used in the cli

use std::io;

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
                        println!("Invalid input. Try again. ");
                    }
                }
                None => {
                    println!(
                        "Invalid input. Try again. The input shall be in the format A1 or 1A."
                    );
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

    let (col, row) = match (chars[0], chars[1]) {
        ('A'..='C', '1'..='3') => (chars[0] as u8 - b'A', chars[1] as u8 - b'1'),
        ('1'..='3', 'A'..='C') => (chars[1] as u8 - b'A', chars[0] as u8 - b'1'),
        _ => return None,
    };
    print!("{} {} ", row, col);
    Some(row as usize * 3 + col as usize)
}
