use std::{
    collections::LinkedList,
    rc::Rc,
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
        time_limit: u32,
    ) {
        let is_queue = frontier_type == FrontierType::Queue;
        let mut final_result: Rc<Board> = Rc::new(Board::new());
        let mut count = 1;
        let start = Instant::now();
        let process = Process::current().unwrap();
        let mut memory_usage_in_bytes: u64 = 0;

        'outer: while depth_limit < 33 {
            let mut frontier_list: LinkedList<Rc<Board>> = LinkedList::new();
            self.generate_possible_moves(&method, &mut frontier_list);
            let mut best_board: Rc<Board> = Rc::new(Board::new());
            while !frontier_list.is_empty() {
                count += 1;

                let current = if is_queue {
                    frontier_list.pop_front().unwrap()
                } else {
                    frontier_list.pop_back().unwrap()
                };
                if count % 50_000 == 0 {
                    memory_usage_in_bytes = process.memory_info().unwrap().rss();
                    if memory_usage_in_bytes > 1 << 33 {
                        println!("Memory limit exceeded.");
                        break 'outer;
                    }
                    if time_limit * 60 < start.elapsed().as_secs() as u32 {
                        println!("Time limit exceeded.");
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
                    );
                }

                if best_board.depth <= current.depth {
                    best_board = Rc::clone(&current);
                    final_result = Rc::clone(&current);
                }
                if current.is_goal_state() {
                    final_result = current;
                    break 'outer;
                }
                if current.depth < depth_limit {
                    current.generate_possible_moves(&method, &mut frontier_list);
                }
            }
            depth_limit += 1;
        }
        let elapsed_time = start.elapsed();
        let mut iterator = final_result.as_ref();
        let mut result_states: Vec<Board> = Vec::new();
        while {
            iterator = iterator.parent.as_ref().unwrap();
            result_states.push(iterator.to_owned());
            !iterator.parent.is_none()
        } {}
        final_result.print_board(
            count,
            final_result.depth,
            false,
            elapsed_time,
            memory_usage_in_bytes,
        );
        for state in result_states {
            state.print_board(0, state.depth, false, Duration::from_secs(0), 0);
        }
    }
}
