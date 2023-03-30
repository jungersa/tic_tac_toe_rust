use super::models::{self, Mark};

/// Validates a game state and returns an error message if the state is invalid.
pub(crate) fn validate_game_state(game_state: &models::GameState) -> Result<(), String> {
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
fn validate_number_of_marks(grid: &models::Grid) -> Result<(), String> {
    let cross_count = grid.cross_count();
    let naught_count = grid.naught_count();
    if cross_count.abs_diff(naught_count) > 1 {
        return Err(String::from("Wrong number of Naughts and Crosses"));
    }
    Ok(())
}

/// Validates the starting mark of a game and returns an error message if the mark is invalid.
fn validate_starting_mark(grid: &models::Grid, starting_mark: &models::Mark) -> Result<(), String> {
    let cross_count = grid.cross_count();
    let naught_count = grid.naught_count();
    if (cross_count > naught_count && starting_mark == &models::Mark::Naught)
        || (cross_count < naught_count && starting_mark == &models::Mark::Cross)
    {
        return Err(String::from("Wrong starting mark"));
    }
    Ok(())
}

/// Validates the winner of a game and returns an error message if the winner is invalid.
fn validate_winner(
    grid: &models::Grid,
    starting_mark: &models::Mark,
    winner: Option<models::Mark>,
) -> Result<(), String> {
    if let Some(winner_mark) = winner {
        if winner_mark == Mark::Cross {
            if starting_mark == &Mark::Cross {
                if grid.cross_count() <= grid.naught_count() {
                    return Err(String::from("Wrong winner mark"));
                }
            } else {
                if grid.cross_count() != grid.naught_count() {
                    return Err(String::from("Wrong winner mark"));
                }
            }
        } else if winner_mark == Mark::Naught {
            if starting_mark == &Mark::Naught {
                if grid.naught_count() <= grid.cross_count() {
                    return Err(String::from("Wrong winner mark"));
                }
            } else {
                if grid.naught_count() != grid.cross_count() {
                    return Err(String::from("Wrong winner mark"));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_number_of_marks_valid() {
        let grid = models::Grid::new(Some([
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_empty(),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_empty(),
            models::Cell::new_empty(),
        ]));
        let game_state = models::GameState::new(grid, None).unwrap();
        assert!(validate_number_of_marks(game_state.grid()).is_ok());
    }

    #[test]
    fn test_validate_number_of_marks_fail() {
        let grid = models::Grid::new(Some([
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_empty(),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_empty(),
            models::Cell::new_empty(),
            models::Cell::new_empty(),
        ]));
        assert_eq!(
            validate_number_of_marks(&grid),
            Err(String::from("Wrong number of Naughts and Crosses"))
        );
    }

    #[test]
    fn test_validate_starting_mark_valid() {
        let grid = models::Grid::new(Some([
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_empty(),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_empty(),
        ]));
        let game_state = models::GameState::new(grid, Some(models::Mark::Cross)).unwrap();
        assert!(validate_starting_mark(game_state.grid(), game_state.starting_mark()).is_ok());
    }

    #[test]
    fn test_validate_starting_mark_fail() {
        let grid = models::Grid::new(Some([
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_empty(),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_empty(),
        ]));
        assert_eq!(
            validate_starting_mark(&grid, &models::Mark::Naught),
            Err(String::from("Wrong starting mark"))
        );
    }

    #[test]
    fn test_validate_winner_valid() {
        let grid = models::Grid::new(Some([
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_empty(),
        ]));
        let game_state = models::GameState::new(grid, Some(models::Mark::Naught)).unwrap();
        assert!(validate_winner(
            game_state.grid(),
            game_state.starting_mark(),
            Some(Mark::Cross)
        )
        .is_ok());
    }

    #[test]
    fn test_validate_winner_fail() {
        let grid = models::Grid::new(Some([
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Cross),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_marked(Mark::Naught),
            models::Cell::new_empty(),
            models::Cell::new_empty(),
        ]));
        let game_state = models::GameState::new(grid, Some(models::Mark::Naught)).unwrap();
        assert!(validate_winner(
            game_state.grid(),
            game_state.starting_mark(),
            Some(Mark::Naught)
        )
        .is_err());
    }
}