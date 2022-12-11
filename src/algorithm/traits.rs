use std::{
    io,
    rc::Rc,
    thread::{self, JoinHandle},
    time::Duration,
};

use psutil::process::Process;

use crate::peg_solitaire::Board;

use super::enums::{FrontierType, Method};

pub trait Algorithm {
    fn solve(
        &self,
        frontier_type: FrontierType,
        method: Method,
        initial_depth_limit: u8,
        time_limit: u16,
    );
    fn get_full_name_of_algorithm(abbreviation: &str) -> &str {
        match abbreviation {
            "bfs" => "Breadth-First Search",
            "dfs" => "Depth-First Search",
            "iterative-dfs" => "Itrative Deepening Search",
            "random-dfs" => "Depth-First Search with Random Selection",
            "heuristic-dfs" => "Depth-First Search with a Node Selection Heuristic",
            _ => "Not Found",
        }
    }
    fn print_solution(
        best_board: Rc<Board>,
        finish_state: u8,
        count: u64,
        elapsed_time: Duration,
        memory_usage_in_bytes: u64,
        frontier_list_max_size: usize,
    ) {
        let is_goal_string = if *best_board.is_solution.borrow() {
            format!(
                "Sub-optimum Solution Found with {} remaining pegs. Do you want to print the solution? (y/n)",
                32 - best_board.depth
            )
        } else {
            format!("No solution found")
        };
        match finish_state {
            1 => {
                println!(
                    "\x1B[2K\x1B[JOptimum solution found. Would you like to print the solution? (y/n)"
                );
            }
            2 => {
                println!("\x1B[2K\x1B[JTime Limit - {}", is_goal_string);
            }
            3 => {
                println!("\x1B[2K\x1B[JOut of Memory - {}", is_goal_string);
            }
            _ => {}
        };
        if !*best_board.is_solution.borrow() {
            return;
        }
        let mut iterator = best_board.as_ref();
        let mut result_states: Vec<Board> = Vec::new();
        result_states.push(best_board.as_ref().clone());
        while {
            iterator = iterator.parent.as_ref().unwrap();
            result_states.push(iterator.to_owned());
            !iterator.parent.is_none()
        } {}
        let mut choice = String::new();
        while {
            io::stdin().read_line(&mut choice).unwrap();
            match choice.trim() {
                "y" | "n" => false,
                _ => true,
            }
        } {}
        if choice.trim() == "n" {
            return;
        }
        for state in result_states.iter().rev() {
            thread::sleep(Duration::from_millis(500));
            state.print_board(
                count,
                0,
                true,
                elapsed_time,
                memory_usage_in_bytes,
                frontier_list_max_size,
                false,
            );
        }
        print!("\x1B[13B")
    }
    fn are_constraints_satisfied(
        timing_thread: &JoinHandle<()>,
        finish_state: &mut u8,
        count: &u64,
        memory_usage_in_bytes: &mut u64,
        process: &Process,
    ) -> bool {
        if timing_thread.is_finished() {
            *finish_state = 2;
            return false;
        }
        if count % 50_000 == 0 {
            *memory_usage_in_bytes = process.memory_info().unwrap().vms();
            if *memory_usage_in_bytes > 1 << 33 {
                *finish_state = 3;
                return false;
            }
        }
        true
    }
}
