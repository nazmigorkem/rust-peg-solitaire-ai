use core::time;
use std::collections::VecDeque;

use crate::solitaire::Board;

pub trait BFS {
    fn solve_bfs(&self) -> i32;
}

impl BFS for Board {
    fn solve_bfs(&self) -> i32 {
        let mut frontier_list: VecDeque<Board> = VecDeque::new();
        self.generate_possible_moves(false)
            .iter()
            .for_each(|x| frontier_list.push_back(x.clone()));
        let mut count = 1;
        let mut final_result: Board = Board::new();
        while !frontier_list.is_empty() {
            count += 1;

            let current = frontier_list.pop_front().unwrap();
            if count % 50_000 == 0 {
                println!("{}", count);
                current.print_board();
            }
            // std::thread::sleep(time::Duration::from_millis(1000));
            if current.is_goal_state() {
                final_result = current;
                break;
            }

            current
                .generate_possible_moves(false)
                .iter()
                .for_each(|x| frontier_list.push_back(x.clone()));
        }
        final_result.print_board();
        return count;
    }
}
