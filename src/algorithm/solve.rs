use std::{collections::LinkedList, rc::Rc};

use crate::peg_solitaire::Board;

use super::enums::{Algorithm, FrontierType, Method};

impl Algorithm for Board {
    fn solve(&self, frontier_type: FrontierType, method: Method, mut depth_limit: u8) {
        let is_random = method == Method::Random;
        let is_using_heuristic = method == Method::Heuristic;
        let is_queue = frontier_type == FrontierType::Queue;
        let mut final_result: Rc<Board> = Rc::new(Board::new());
        let mut count = 1;

        'outter: while depth_limit < 33 {
            let mut frontier_list: LinkedList<Rc<Board>> = LinkedList::new();
            if is_using_heuristic {
                self.generate_possible_moves(is_random, &mut frontier_list);
            } else {
                self.generate_possible_moves(is_random, &mut frontier_list);
            }
            let mut best_board: Rc<Board> = Rc::new(Board::new());
            depth_limit += 1;
            while !frontier_list.is_empty() {
                count += 1;

                let current = if is_queue {
                    frontier_list.pop_front().unwrap()
                } else {
                    frontier_list.pop_back().unwrap()
                };
                if count % 1_000_000 == 0 {
                    best_board.print_board(count, best_board.depth, true);
                }

                if best_board.depth <= current.depth {
                    best_board = Rc::clone(&current);
                    final_result = Rc::clone(&current);
                }
                if current.is_goal_state() {
                    final_result = current;
                    break 'outter;
                }
                if is_using_heuristic {
                    current.generate_possible_moves(is_random, &mut frontier_list);
                } else {
                    if current.depth < depth_limit {
                        current.generate_possible_moves(is_random, &mut frontier_list);
                    }
                }
            }
        }
        let mut iterator = final_result.parent.as_ref().unwrap();
        while {
            iterator.print_board(0, iterator.depth, false);
            iterator = iterator.parent.as_ref().unwrap();
            !iterator.parent.is_none()
        } {}
        final_result.print_board(count, final_result.depth, false);
    }
}
