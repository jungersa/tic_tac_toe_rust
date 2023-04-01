//! The `Grid` module contains the `Grid` struct and its methods.
//! The `Grid` struct represents the game board grid.
//! It contains a list of `Cell` of size `Grid::SIZE`.
use crate::logic::{Cell, Mark};

/// Represents the game board grid.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Grid {
    cells: [Cell; Grid::SIZE],
}

impl Grid {
    pub const WIDTH: usize = 3;
    pub const SIZE: usize = Grid::WIDTH * Grid::WIDTH;

    /// Creates a new `Grid` with the given list of `Cell`.
    ///
    /// If no list of `Cell` is provided, the default is a list of empty cells.
    ///
    /// # Arguments
    ///
    /// * `cells` - The list of cells size of Grid::SIZE.
    ///
    pub(crate) fn new(cells: Option<[Cell; Grid::SIZE]>) -> Self {
        if let Some(cell) = cells {
            Self { cells: cell }
        } else {
            Self {
                cells: [Cell::new_empty(); Grid::SIZE],
            }
        }
    }

    /// Returns the number of empty cells in the grid.
    pub(crate) fn empty_count(&self) -> usize {
        self.cells.iter().filter(|&cell| cell.is_vacant()).count()
    }

    /// Returns the number of cells which are naught in the grid.
    pub(crate) fn naught_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|&cell| cell.is_occupied_by(Mark::Naught))
            .count()
    }

    /// Returns the number of cells which are cross in the grid.
    pub(crate) fn cross_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|&cell| cell.is_occupied_by(Mark::Cross))
            .count()
    }

    pub(crate) fn cells(&self) -> &[Cell] {
        &self.cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_count_full() {
        let grid = Grid {
            cells: [Cell::new_empty(); Grid::SIZE],
        };
        assert_eq!(grid.empty_count(), Grid::SIZE);
    }

    #[test]
    fn test_empty_count() {
        let grid = Grid {
            cells: [
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Cross),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Naught),
            ],
        };
        assert_eq!(grid.empty_count(), 5);
    }

    #[test]
    fn test_naught_count() {
        let grid = Grid {
            cells: [
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Cross),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Naught),
            ],
        };
        assert_eq!(grid.naught_count(), 2);
    }

    #[test]
    fn test_cross_count() {
        let grid = Grid {
            cells: [
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Cross),
                Cell::new_marked(Mark::Cross),
                Cell::new_empty(),
                Cell::new_empty(),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Naught),
                Cell::new_marked(Mark::Naught),
            ],
        };
        assert_eq!(grid.cross_count(), 4);
    }

    #[test]
    fn test_new_with_cells() {
        let cells = [
            Cell::new_marked(Mark::Cross),
            Cell::new_marked(Mark::Naught),
            Cell::new_empty(),
            Cell::new_marked(Mark::Cross),
            Cell::new_empty(),
            Cell::new_empty(),
            Cell::new_empty(),
            Cell::new_empty(),
            Cell::new_marked(Mark::Naught),
        ];
        let grid = Grid::new(Some(cells));

        assert_eq!(grid.cells.len(), 9);
        assert!(grid.cells[0].is_occupied_by(Mark::Cross));
        assert!(grid.cells[1].is_occupied_by(Mark::Naught));
        assert!(grid.cells[2].is_vacant());
        assert!(grid.cells[3].is_occupied_by(Mark::Cross));
        assert!(grid.cells[4].is_vacant());
        assert!(grid.cells[5].is_vacant());
        assert!(grid.cells[6].is_vacant());
        assert!(grid.cells[7].is_vacant());
        assert!(grid.cells[8].is_occupied_by(Mark::Naught));
    }

    #[test]
    fn test_new_without_cells() {
        let grid = Grid::new(None);

        assert_eq!(grid.cells.len(), 9);
        for cell in grid.cells.iter() {
            assert!(cell.is_vacant());
        }
    }
}
