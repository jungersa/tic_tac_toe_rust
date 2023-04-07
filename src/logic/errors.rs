use thiserror::Error;

use super::Mark;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration Error: `{0}`")]
    ConfigError(String),
    #[error("Move Error")]
    MoveError(MoveError),
    #[error("Validation Error")]
    ValidationError(ValidationError),
}

#[derive(Error, Debug)]
pub enum MoveError {
    #[error("No more possible moves")]
    NoPossibleMoves,
    #[error("It's the other player's turn `{0}`")]
    NotYourTurn(Mark),
    #[error("Cell `{0}`  is already marked")]
    CellAlreadyMarked(usize),
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Wrong number of naughts and crosses `{0}` `{1}`, expected 0 or 1 difference")]
    WrongNumberOfNaughtsAndCrosses(usize, usize),
    #[error("Wrong starting mark `{0}`, expected the other mark")]
    WrongStartingMark(Mark),
    #[error("Wrong winner mark `{0}`, expected the other mark")]
    WrongWinnerMark(Mark),
}
