use std::{collections::VecDeque, thread, time::Duration};

use crate::solitaire::{enums::DFS, Board};

impl DFS for Board {
    fn solve_dfs(&self, is_random: bool) {
        let mut frontier_list: VecDeque<Board> = VecDeque::new();
        self.generate_possible_moves(is_random)
            .iter()
            .for_each(|x| frontier_list.push_back(x.clone()));
        let mut count = 1;
        let mut final_result: Board = Board::new();
        while !frontier_list.is_empty() {
            count += 1;

            let current = frontier_list.pop_back().unwrap();
            if count % 50_000 == 0 {
                current.print_board(count, current.depth);
            }
            // thread::sleep(Duration::from_millis(1000));
            if current.is_goal_state() {
                final_result = current;
                break;
            }

            current
                .generate_possible_moves(is_random)
                .iter()
                .for_each(|x| frontier_list.push_back(x.clone()));
        }
        final_result.print_board(count, final_result.depth);
    }
}
