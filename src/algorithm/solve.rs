use std::{
    collections::LinkedList,
    io,
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

use psutil::process::Process;

use crate::peg_solitaire::Board;

use super::enums::{Algorithm, FrontierType, Method};

impl Algorithm for Board {
    fn solve(
        &self,
        frontier_type: FrontierType,
        method: Method,
        mut depth_limit: u8,
        time_limit: u16,
    ) {
        let is_queue = frontier_type == FrontierType::Queue;
        let mut final_result: Rc<Board> = Rc::new(Board::new());
        let mut count = 1;
        let start = Instant::now();
        let process = Process::current().unwrap();
        let mut memory_usage_in_bytes: u64 = 0;
        let time_limit_in_seconds = time_limit * 60;
        let mut best_board: Rc<Board> = Rc::new(Board::new());
        let timing_thread = thread::spawn(move || {
            thread::sleep(Duration::from_secs(time_limit_in_seconds as u64));
        });
        let mut frontier_list_max_size = 0;
        let mut finish_state = 0;
        'outer: while depth_limit < 33 {
            let mut frontier_list: LinkedList<Rc<Board>> = LinkedList::new();
            self.generate_possible_moves(&method, &mut frontier_list);
            while !frontier_list.is_empty() {
                count += 1;
                let current = if is_queue {
                    frontier_list.pop_front().unwrap()
                } else {
                    frontier_list.pop_back().unwrap()
                };
                if current.depth < depth_limit {
                    current.generate_possible_moves(&method, &mut frontier_list);
                }
                if timing_thread.is_finished() {
                    finish_state = 2;
                    break 'outer;
                }
                if count % 50_000 == 0 {
                    memory_usage_in_bytes = process.memory_info().unwrap().vms();
                    if memory_usage_in_bytes > 1 << 33 {
                        finish_state = 3;
                        break 'outer;
                    }
                }
                if count % 1_000_000 == 0 {
                    best_board.print_board(
                        count,
                        best_board.depth,
                        true,
                        start.elapsed(),
                        memory_usage_in_bytes,
                        frontier_list_max_size,
                    );
                }
                if best_board.depth < current.depth {
                    best_board = Rc::clone(&current);
                    final_result = Rc::clone(&current);
                }
                if frontier_list_max_size < frontier_list.len() {
                    frontier_list_max_size = frontier_list.len();
                }
                if current.is_goal_state() {
                    finish_state = 1;
                    memory_usage_in_bytes = process.memory_info().unwrap().vms();
                    final_result = current;
                    break 'outer;
                }
            }
            depth_limit += 1;
        }
        let elapsed_time = start.elapsed();
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
        let mut iterator = final_result.as_ref();
        let mut result_states: Vec<Board> = Vec::new();
        result_states.push(final_result.as_ref().clone());
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
                state.depth,
                true,
                elapsed_time,
                memory_usage_in_bytes,
                frontier_list_max_size,
            );
        }
        print!("\x1B[13B")
    }
}
