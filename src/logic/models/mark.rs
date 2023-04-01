/// Represents a mark on the board in a Tic Tac Toe game.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Mark {
    /// The mark representing a cross, which is denoted by the string "X".
    Cross,
    /// The mark representing a naught, which is denoted by the string "O".
    Naught,
}

impl Mark {
    /// Returns a new instance of the enum with the opposite variant.
    pub(super) fn other(&self) -> Self {
        match self {
            Mark::Cross => Mark::Naught,
            Mark::Naught => Mark::Cross,
        }
    }
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Mark::Cross => write!(f, "X"),
            Mark::Naught => write!(f, "O"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_other_naught() {
        let cross = Mark::Cross;
        let naught = cross.other();
        assert_eq!(naught, Mark::Naught);
    }

    #[test]
    fn test_other_cross() {
        let naught = Mark::Naught;
        let cross = naught.other();
        assert_eq!(cross, Mark::Cross);
    }
}
