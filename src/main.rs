mod algorithm;
mod peg_solitaire;

use std::env::args;

use crate::algorithm::enums::{Algorithm, FrontierType, Method};

use self::peg_solitaire::Board;

fn main() {
    let board = Board::new();
    let args: Vec<String> = args().collect();
    match &args[1][2..] {
        "bfs" => {
            board.solve(FrontierType::Queue, Method::Ordered, 32);
        }
        "dfs" => {
            board.solve(FrontierType::Stack, Method::Ordered, 32);
        }
        "iterative-dfs" => {
            board.solve(FrontierType::Stack, Method::Ordered, 1);
        }
        "random-dfs" => {
            board.solve(FrontierType::Stack, Method::Random, 32);
        }
        "heuristic-dfs" => {
            board.solve(FrontierType::Stack, Method::Heuristic, 32);
        }
        _ => {}
    }
}
