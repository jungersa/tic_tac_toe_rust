//! This module contains functions to validate game states.
//! The functions in this module are used to validate the game state before the game starts.
//! And they are used to validate the game state after each move.

use super::{GameState, Grid, Mark};

/// Validates a game state and returns an error message if the state is invalid.
///
/// # Arguments
///
/// * `game_state` - The game state to validate.
pub(crate) fn validate_game_state(game_state: &GameState) -> Result<(), String> {
    validate_number_of_marks(game_state.grid())?;
    validate_starting_mark(game_state.grid(), game_state.starting_mark())?;
    validate_winner(
        game_state.grid(),
        game_state.starting_mark(),
        game_state.winner_mark(),
    )?;
    Ok(())
}

/// Validates the number of marks in a game and returns an error message if the number is invalid.
///
/// The number of marks is invalid if:
/// - The number of marks of the Cross mark is less than the number of marks of the Naught mark by more than 1.
/// - The number of marks of the Naught mark is less than the number of marks of the Cross mark by more than 1.
///
/// # Arguments
///
/// * `grid` - The grid of the game.
fn validate_number_of_marks(grid: &Grid) -> Result<(), String> {
    let cross_count = grid.cross_count();
    let naught_count = grid.naught_count();
    if cross_count.abs_diff(naught_count) > 1 {
        return Err(String::from("Wrong number of Naughts and Crosses"));
    }
    Ok(())
}

/// Validates the starting mark of a game and returns an error message if the mark is invalid.
/// The starting mark is invalid if:
/// - The number of marks of the starting mark is greater than the number of marks of the other mark.
/// - The number of marks of the starting mark is less than the number of marks of the other mark by more than 1.
///
/// # Arguments
///
/// * `grid` - The grid of the game.
/// * `starting_mark` - The starting mark of the game.
fn validate_starting_mark(grid: &Grid, starting_mark: &Mark) -> Result<(), String> {
    let cross_count = grid.cross_count();
    let naught_count = grid.naught_count();
    if (cross_count > naught_count && starting_mark == &Mark::Naught)
        || (cross_count < naught_count && starting_mark == &Mark::Cross)
    {
        return Err(String::from("Wrong starting mark"));
    }
    Ok(())
}

/// Validates the winner of a game and returns an error message if the winner is invalid.
///
/// The winner is invalid if:
/// - The winner is not the starting mark and the number of marks of the winner is not greater than the number of marks of the other mark.
/// - The winner is the starting mark and the number of marks of the winner is not greater than the number of marks of the other mark.
///
/// # Arguments
///
/// * `grid` - The grid of the game.
/// * `starting_mark` - The starting mark of the game.
/// * `winner` - The winner of the game.
fn validate_winner(grid: &Grid, starting_mark: &Mark, winner: Option<Mark>) -> Result<(), String> {
    if let Some(winner_mark) = winner {
        if winner_mark == Mark::Cross {
            if starting_mark == &Mark::Cross {
                if grid.cross_count() <= grid.naught_count() {
                    return Err(String::from("Wrong winner mark"));
                }
            } else if grid.cross_count() != grid.naught_count() {
                return Err(String::from("Wrong winner mark"));
            }
        } else if winner_mark == Mark::Naught {
            if starting_mark == &Mark::Naught {
                if grid.naught_count() <= grid.cross_count() {
                    return Err(String::from("Wrong winner mark"));
                }
            } else if grid.naught_count() != grid.cross_count() {
                return Err(String::from("Wrong winner mark"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::logic::Cell;

    use super::*;

    #[test]
    fn test_validate_number_of_marks_valid() {
        let grid = Grid::new(Some([
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Cross),
            Cell::new_empty(),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_empty(),
            Cell::new_empty(),
        ]));
        let game_state = GameState::new(grid, None).unwrap();
        assert!(validate_number_of_marks(game_state.grid()).is_ok());
    }

    #[test]
    fn test_validate_number_of_marks_fail() {
        let grid = Grid::new(Some([
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Cross),
            Cell::new_empty(),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_marked(Mark::Cross),
            Cell::new_empty(),
            Cell::new_empty(),
            Cell::new_empty(),
        ]));
        assert_eq!(
            validate_number_of_marks(&grid),
            Err(String::from("Wrong number of Naughts and Crosses"))
        );
    }

    #[test]
    fn test_validate_starting_mark_valid() {
        let grid = Grid::new(Some([
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Cross),
            Cell::new_empty(),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_empty(),
        ]));
        let game_state = GameState::new(grid, Some(Mark::Cross)).unwrap();
        assert!(validate_starting_mark(game_state.grid(), game_state.starting_mark()).is_ok());
    }

    #[test]
    fn test_validate_starting_mark_fail() {
        let grid = Grid::new(Some([
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Cross),
            Cell::new_empty(),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_empty(),
        ]));
        assert_eq!(
            validate_starting_mark(&grid, &Mark::Naught),
            Err(String::from("Wrong starting mark"))
        );
    }

    #[test]
    fn test_validate_winner_valid() {
        let grid = Grid::new(Some([
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Naught),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_empty(),
        ]));
        let game_state = GameState::new(grid, Some(Mark::Naught)).unwrap();
        assert!(validate_winner(
            game_state.grid(),
            game_state.starting_mark(),
            Some(Mark::Cross)
        )
        .is_ok());
    }

    #[test]
    fn test_validate_winner_fail() {
        let grid = Grid::new(Some([
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Naught),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_empty(),
        ]));
        let game_state = GameState::new(grid, Some(Mark::Naught)).unwrap();
        assert!(validate_winner(
            game_state.grid(),
            game_state.starting_mark(),
            Some(Mark::Naught)
        )
        .is_err());
    }
}
