mod algorithm;
mod solitaire;

use std::{collections::VecDeque, io};

use solitaire::enums::DFSWithHeuristic;

use crate::solitaire::enums::{IterativeDeepining, BFS, DFS};

use self::solitaire::Board;

fn main() {
    let board = Board::new();
    board.print_board(0, 0);
    // println!(
    //     "Select one of the methods from a to e:\na) Breadth-First Search\nb) Depth-First Search\nc) Iterative Deepening Search\nd) Depth-First Search with Random Selection\ne) Depth-First Search with a Node Selection Heuristic\nf) Exit"
    // );
    // loop {
    //     let mut selection = String::new();

    //     io::stdin()
    //         .read_line(&mut selection)
    //         .expect("Failed to read line");
    //     let mut initial_frontier: VecDeque<Board> = VecDeque::new();
    //     board.generate_possible_moves(false, &mut initial_frontier);
    //     let initial_state = initial_frontier.pop_front().unwrap();
    //     match selection.trim() {
    //         "a" => {
    //             initial_state.solve_bfs();
    //         }
    //         "b" => {
    //             initial_state.solve_dfs(false);
    //         }
    //         "c" => {
    //             initial_state.solve_iterative_deepening();
    //         }
    //         "d" => {
    //             initial_state.solve_dfs(true);
    //         }
    //         "e" => {
    //             initial_state.solve_dfs_with_heuristic();
    //         }
    //         "f" => {
    //             println!("Exitting from the program.");
    //             break;
    //         }
    //         _ => {}
    //     }
    // }
}
