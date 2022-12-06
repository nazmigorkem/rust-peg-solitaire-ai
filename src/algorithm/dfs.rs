use std::{borrow::Borrow, cell::RefCell, collections::VecDeque, ops::Deref, rc::Rc};

use crate::solitaire::{enums::DFS, Board};

impl DFS for Board {
    fn solve_dfs(&self, is_random: bool) {
        let mut frontier_list: VecDeque<Rc<Board>> = VecDeque::new();
        self.generate_possible_moves(is_random, &mut frontier_list);
        let mut count = 1;
        let mut final_result: Rc<Board> = Rc::new(Board::new());
        let mut best_move: Rc<Board> = Rc::new(Board::new());
        while !frontier_list.is_empty() {
            count += 1;

            let current = frontier_list.pop_back().unwrap();
            if count % 1_000_000 == 0 {
                best_move.print_board(count, best_move.depth, true);
            }

            if best_move.depth <= current.depth {
                best_move = Rc::clone(&current);
                final_result = Rc::clone(&current);
            }
            if current.is_goal_state() {
                final_result = current;
                break;
            }
            current.generate_possible_moves(is_random, &mut frontier_list);
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
