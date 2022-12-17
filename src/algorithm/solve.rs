use std::{collections::LinkedList, rc::Rc, sync::mpsc, time::Instant};

use psutil::process::Process;

use crate::peg_solitaire::Board;

use super::{
    enums::{FrontierType, Method},
    traits::{Algorithm, Check},
};

impl Algorithm for Board {
    fn solve(
        &self,
        frontier_type: FrontierType,
        method: Method,
        mut depth_limit: u8,
        time_limit: u16,
    ) {
        // initiliazing local variables for algorithm and creating threads for constraint checks
        let mut count = 0;
        let start = Instant::now();
        let process = Process::current().unwrap();
        let mut memory_usage_in_bytes: u64 = 0;
        let time_limit_in_seconds = time_limit * 60;
        let mut best_board: Rc<Board> = Rc::new(Board::new());
        // for thread communication, used mpsc channels
        let (memory_sender, memory_receiver) = mpsc::channel();
        let (time_checker, time_receiver) = mpsc::channel();
        Board::timing_thread(time_limit_in_seconds as u64, time_checker);
        Board::memory_thread(memory_sender);
        let mut frontier_list_max_size = 0;
        let mut finish_state = 0;
        // main loop starts here
        let mut frontier_list: LinkedList<Rc<Board>> = LinkedList::new();
        'outer: loop {
            self.generate_possible_moves(&method, &mut frontier_list, depth_limit);
            count += 1;
            while !frontier_list.is_empty() {
                count += 1;
                // popping the current node via checking the type of the algorithm
                // if it is BFS => use queue
                // else => use stack
                let current = match frontier_type {
                    FrontierType::Queue => frontier_list.pop_front(),
                    FrontierType::Stack => frontier_list.pop_back(),
                }
                .unwrap();
                // checks the depth limit for iterative deepening
                // if default DFS is used, then depth_limit is given 32 which is the solution depth
                current.generate_possible_moves(&method, &mut frontier_list, depth_limit);

                // check whether the constraints are satisfied
                // if not break the outer loop, so program can stop
                if !Board::are_constraints_satisfied(
                    time_receiver.try_recv().unwrap_or(false),
                    &mut finish_state,
                    memory_receiver
                        .try_recv()
                        .unwrap_or((memory_usage_in_bytes, false)),
                    &mut memory_usage_in_bytes,
                ) {
                    break 'outer;
                }
                // update best board depending on its depth
                if *current.is_solution.borrow() && best_board.depth < current.depth {
                    best_board = Rc::clone(&current);
                }
                if frontier_list_max_size < frontier_list.len() {
                    frontier_list_max_size = frontier_list.len();
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
