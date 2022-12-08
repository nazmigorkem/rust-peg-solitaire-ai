mod algorithm;
mod peg_solitaire;

use std::env::args;

use crate::algorithm::enums::{Algorithm, FrontierType, Method};

use self::peg_solitaire::Board;

fn main() {
    let board = Board::new();
    let arguments: Vec<String> = args().collect();

    if arguments.len() < 3 {
        println!("Run the program via providing algorithm name and time limit in minutes.");
        return;
    }
    let mut time_limit = 0;

    let mut search_algorithm = "";

    for (i, argument) in arguments.iter().enumerate() {
        if i == 0 {
            continue;
        }

        match argument.as_str() {
            "--search-algorithm" | "-s" => {
                search_algorithm = &arguments[i + 1][..];
            }
            "--time-limit" | "-t" => {
                time_limit = match arguments[i + 1][..].parse::<u16>() {
                    Ok(number) => number,
                    Err(_) => {
                        println!("Please provide valid time limit in minutes.");
                        return;
                    }
                };
            }
            _ => {}
        }
    }

    match search_algorithm {
        "bfs" => {
            board.solve(FrontierType::Queue, Method::Ordered, 32, time_limit);
        }
        "dfs" => {
            board.solve(FrontierType::Stack, Method::Ordered, 32, time_limit);
        }
        "iterative-dfs" => {
            board.solve(FrontierType::Stack, Method::Ordered, 1, time_limit);
        }
        "random-dfs" => {
            board.solve(FrontierType::Stack, Method::Random, 32, time_limit);
        }
        "heuristic-dfs" => {
            board.solve(FrontierType::Stack, Method::Heuristic, 32, time_limit);
        }
        _ => {}
    }
}
