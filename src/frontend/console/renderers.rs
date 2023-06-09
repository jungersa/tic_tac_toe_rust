//! The renderer which is used in the cli interface
use crate::{
    game::renderers::Renderer,
    logic::{GameState, Grid},
};

pub struct ConsoleRenderer;

impl Renderer for ConsoleRenderer {
    /// Render the game with the curent `GameState`
    ///
    /// # Arguments
    ///
    /// * game_state - the curent `GameState` which will be rendered
    fn render(&self, game_state: &GameState) {
        if game_state.game_not_started() {
            println!("Nice to see you play");
        }
        clear_screen();
        print_game(game_state.grid());

        if game_state.game_over() {
            match game_state.winner_mark() {
                Some(mark) => {
                    println!("{} wins!", mark);
                    match game_state.winning_indexes() {
                        Some(indexes) => println!("The winning indexes are: {:?}", indexes),
                        None => todo!("No winning indexes"),
                    }
                }
                None => print!("No one wins this time"),
            }
        }
    }
}

/// Clear the terminal screen
fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

/// Print the grid to the standard output
///
/// # Arguments
///
/// * grid - The `Grid` to be printed on the terminal
fn print_game(grid: &Grid) {
    let output = format!(
        r#"
        A   B   C
        ------------
     1 ┆  {0} │ {1} │ {2}
       ┆ ───┼───┼───
     2 ┆  {3} │ {4} │ {5}
       ┆ ───┼───┼───
     3 ┆  {6} │ {7} │ {8}
    "#,
        grid.cells()[0],
        grid.cells()[1],
        grid.cells()[2],
        grid.cells()[3],
        grid.cells()[4],
        grid.cells()[5],
        grid.cells()[6],
        grid.cells()[7],
        grid.cells()[8],
    );
    println!("{}", output);
}
