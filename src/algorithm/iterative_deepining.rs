use std::collections::VecDeque;

use crate::solitaire::Board;

pub trait IterativeDeepining {
    fn solve_iterative_deepening(&self) -> i32;
}

impl IterativeDeepining for Board {
    fn solve_iterative_deepening(&self) -> i32 {
        let mut count = 1;
        let mut final_result: Board = Board::new();
        let depth_limit = 31;
        let mut current_limit = 1;
        'main: while current_limit <= depth_limit {
            let mut frontier_list: VecDeque<Board> = VecDeque::new();
            self.generate_possible_moves(false)
                .iter()
                .for_each(|x| frontier_list.push_back(x.clone()));
            while !frontier_list.is_empty() {
                count += 1;

                let current = frontier_list.pop_back().unwrap();
                if count % 50_000 == 0 {
                    println!("{} {}", count, current.depth);
                    current.print_board();
                }
                if current.is_goal_state() {
                    final_result = current;
                    break 'main;
                }

                current.generate_possible_moves(false).iter().for_each(|x| {
                    if x.depth <= current_limit {
                        frontier_list.push_back(x.clone())
                    }
                });
            }
            current_limit += 1;
        }
        final_result.print_board();
        return count;
    }
}
