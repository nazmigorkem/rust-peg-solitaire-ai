mod algorithm;
mod solitaire;

use solitaire::enums::DFSWithHeuristic;

use self::solitaire::Board;

fn main() {
    let board = Board::new();
    board.generate_possible_moves(false)[0].solve_dfs_with_heuristic();
}
