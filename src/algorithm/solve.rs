use std::{
    collections::LinkedList,
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

use psutil::process::Process;

use crate::peg_solitaire::Board;

use super::{
    enums::{FrontierType, Method},
    traits::Algorithm,
};

impl Algorithm for Board {
    fn solve(
        &self,
        frontier_type: FrontierType,
        method: Method,
        mut depth_limit: u8,
        time_limit: u16,
    ) {
        let is_queue = frontier_type == FrontierType::Queue;
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
                if !Board::are_constraints_satisfied(
                    &timing_thread,
                    &mut finish_state,
                    &count,
                    &mut memory_usage_in_bytes,
                    &process,
                ) {
                    break 'outer;
                }
                if count % 1_000_000 == 0 {
                    best_board.print_board(
                        count,
                        best_board.depth,
                        true,
                        start.elapsed(),
                        memory_usage_in_bytes,
                        frontier_list_max_size,
                        true,
                    );
                }
                if best_board.depth < current.depth {
                    best_board = Rc::clone(&current);
                }
                if frontier_list_max_size < frontier_list.len() {
                    frontier_list_max_size = frontier_list.len();
                }
                if current.is_goal_state() {
                    finish_state = 1;
                    memory_usage_in_bytes = process.memory_info().unwrap().vms();
                    best_board = Rc::clone(&current);
                    break 'outer;
                }
            }
            depth_limit += 1;
        }
        let elapsed_time = start.elapsed();
        Board::print_solution(
            best_board,
            finish_state,
            count,
            elapsed_time,
            memory_usage_in_bytes,
            frontier_list_max_size,
        )
    }
}
