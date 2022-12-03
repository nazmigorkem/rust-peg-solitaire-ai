mod algorithm;
mod solitaire;

use std::io;

use solitaire::enums::DFSWithHeuristic;

use crate::solitaire::enums::{IterativeDeepining, BFS, DFS};

use self::solitaire::Board;

fn main() {
    let board = Board::new();

    println!(
        "Select one of the methods from a to e:\na) Breadth-First Search\nb) Depth-First Search\nc) Iterative Deepening Search\nd) Depth-First Search with Random Selection\ne) Depth-First Search with a Node Selection Heuristic\nf) Exit"
    );
    loop {
        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");
        let initial_state = &board.generate_possible_moves(false)[0];
        match selection.trim() {
            "a" => {
                initial_state.solve_bfs();
            }
            "b" => {
                initial_state.solve_dfs(false);
            }
            "c" => {
                initial_state.solve_iterative_deepening();
            }
            "d" => {
                initial_state.solve_dfs(true);
            }
            "e" => {
                initial_state.solve_dfs_with_heuristic();
            }
            "f" => {
                println!("Exitting from program.");
                break;
            }
            _ => {}
        }
    }
}
