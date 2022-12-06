mod algorithm;
mod solitaire;

use std::io;

use crate::algorithm::enums::{Algorithm, FrontierType, Method};

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
                board.solve(FrontierType::Queue, Method::Ordered, 32);
            }
            "b" => {
                board.solve(FrontierType::Stack, Method::Ordered, 32);
            }
            "c" => {
                board.solve(FrontierType::Stack, Method::Ordered, 1);
            }
            "d" => {
                board.solve(FrontierType::Stack, Method::Random, 32);
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
