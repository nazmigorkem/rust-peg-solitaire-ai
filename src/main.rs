mod algorithm;
mod solitaire;

use std::{collections::VecDeque, io, rc::Rc};

use crate::solitaire::enums::DFS;

use self::solitaire::Board;

fn main() {
    let board = Board::new();
    println!(
        "Select one of the methods from a to e:\na) Breadth-First Search\nb) Depth-First Search\nc) Iterative Deepening Search\nd) Depth-First Search with Random Selection\ne) Depth-First Search with a Node Selection Heuristic\nf) Exit"
    );
    let mut is_true = false;
    while !is_true {
        let mut selection = String::new();
        is_true = true;
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");
        match selection.trim() {
            "a" => {
                // board.solve_bfs();
            }
            "b" => {
                board.solve_dfs(false);
            }
            "c" => {
                // board.solve_iterative_deepening();
            }
            "d" => {
                board.solve_dfs(true);
            }
            "e" => {
                // board.solve_dfs_with_heuristic();
            }
            "f" => {
                println!("Exitting from the program.");
                break;
            }
            _ => {
                is_true = false;
            }
        }
    }
}
