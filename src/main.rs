mod algorithm;
mod peg_solitaire;

use std::{env::args, time::Duration};

use crate::algorithm::{
    enums::{FrontierType, Method},
    traits::Algorithm,
};

use self::peg_solitaire::Board;

fn main() {
    let board = Board::new(); // initializing board
    let arguments: Vec<String> = args().collect();

    if arguments.len() < 3 {
        println!("Run the program via providing algorithm name and time limit in minutes.");
        return;
    }
    let mut time_limit: u16 = 0;

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
    let algorithm_name = Board::get_full_name_of_algorithm(search_algorithm);
    let algorithm_name_length = algorithm_name.len();
    let time_limit_information_string = format!("Time limit is {time_limit} minute(s).");
    let time_limit_information_length = time_limit_information_string.len();
    let width = std::cmp::max(algorithm_name_length, time_limit_information_length);
    println!(
        "{:=<width$}\n{}\n{}\n{:=<width$}",
        "=", algorithm_name, time_limit_information_string, "=",
    );
    if algorithm_name == "Not Found" {
        return;
    }
    board.print_board(0, 0, true, Duration::from_secs(0), 0, 0, false);
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
