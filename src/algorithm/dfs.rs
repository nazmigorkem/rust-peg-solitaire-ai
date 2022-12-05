use std::{collections::VecDeque, thread, time::Duration};

use crate::solitaire::{enums::DFS, Board};

impl DFS for Board {
    fn solve_dfs(&self, is_random: bool) {
        let mut frontier_list: VecDeque<Board> = VecDeque::new();
        self.generate_possible_moves(is_random, &mut frontier_list);
        let mut count = 1;
        let mut final_result: Board = Board::new();
        let mut best_move = Board::new();
        while !frontier_list.is_empty() {
            count += 1;

            let current = frontier_list.pop_back().unwrap();
            if count % 1_000_000 == 0 {
                best_move.print_board(count, best_move.depth);
            }
            if best_move.depth <= current.depth {
                best_move = current.clone();
                final_result = current.clone();
            }
            if current.is_goal_state() {
                final_result = current;
                break;
            }
            current.generate_possible_moves(is_random, &mut frontier_list);
        }
        final_result.print_board(count, final_result.depth);
    }
}
