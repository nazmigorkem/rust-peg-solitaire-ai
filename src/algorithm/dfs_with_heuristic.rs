use std::{collections::VecDeque, thread, time::Duration};

use crate::solitaire::{enums::DFSWithHeuristic, Board};

impl DFSWithHeuristic for Board {
    fn calculate_heuristic_value(&self) -> u8 {
        let mut result: u8 = 0;
        for (i, j) in self.pegs.clone() {
            let current = if i > 3 { i - 3 } else { 3 - i } + if j > 3 { j - 3 } else { 3 - j };
            if current > result {
                result = current;
            }
        }
        result
    }
    fn generate_possible_moves_with_heuristic(&self) -> Vec<Board> {
        let mut possible_moves = self.generate_possible_moves(false);
        possible_moves.sort();
        possible_moves
    }
    fn solve_dfs_with_heuristic(&self) {
        let mut frontier_list: VecDeque<Board> = VecDeque::new();
        self.generate_possible_moves_with_heuristic()
            .iter()
            .for_each(|x| frontier_list.push_back(x.clone()));
        let mut count = 1;
        let mut final_result: Board = Board::new();
        while !frontier_list.is_empty() {
            count += 1;

            let current = frontier_list.pop_back().unwrap();
            if count % 50_000 == 0 {}
            current.print_board(count, current.depth);
            thread::sleep(Duration::from_millis(1000));
            if current.is_goal_state() {
                final_result = current;
                break;
            }

            current
                .generate_possible_moves_with_heuristic()
                .iter()
                .for_each(|x| frontier_list.push_back(x.clone()));
        }
        final_result.print_board(count, final_result.depth);
    }
}
