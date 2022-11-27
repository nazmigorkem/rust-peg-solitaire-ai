mod algorithm;
mod solitaire;

use crate::algorithm::{bfs::BFS, dfs::DFS, iterative_deepining::IterativeDeepining};

use self::solitaire::Board;

fn main() {
    let board = Board::new();

    println!("{}", board.solve_dfs());
}
