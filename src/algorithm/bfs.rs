use std::collections::VecDeque;

use crate::solitaire::{enums::BFS, Board};

impl BFS for Board {
    fn solve_bfs(&self) {
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
                current.print_board(count, current.depth);
            }
            if current.is_goal_state() {
                final_result = current;
                break;
            }

            current
                .generate_possible_moves(false)
                .iter()
                .for_each(|x| frontier_list.push_back(x.clone()));
        }
        final_result.print_board(count, final_result.depth);
    }
}
