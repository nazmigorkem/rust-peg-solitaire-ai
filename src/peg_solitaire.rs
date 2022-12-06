use std::{
    collections::{BTreeSet, LinkedList},
    rc::Rc,
};

use rand::{seq::SliceRandom, thread_rng};

use crate::algorithm::enums::Method;

#[derive(Debug, Clone)]
pub struct Board {
    pub pegs: BTreeSet<(u8, u8)>,
    pub depth: u8,
    pub heuristic_value: u8,
    pub parent: Option<Rc<Self>>,
}

impl Board {
    pub fn new() -> Board {
        let mut pegs: BTreeSet<(u8, u8)> = BTreeSet::new();
        for i in 0..7 {
            for j in 0..7 {
                if !(i == 3 && j == 3) && !Board::is_out_of_bounds((i, j)) {
                    pegs.insert((i, j));
                }
            }
        }
        let heuristic_value = Board::calculate_heuristic_value(&pegs);
        Board {
            pegs,
            depth: 0,
            parent: None,
            heuristic_value,
        }
    }

    pub fn generate_possible_moves(
        &self,
        method: &Method,
        frontier_list: &mut LinkedList<Rc<Board>>,
    ) {
        let mut outcome_list: Vec<Rc<Board>> = Vec::new();
        for (i, j) in self.pegs.iter().rev() {
            let left_peg = (*i, *j - 1);
            let right_peg = (*i, *j + 1);
            if !Board::is_out_of_bounds(left_peg) && !Board::is_out_of_bounds(right_peg) {
                let left_peg_contain = self.pegs.contains(&left_peg);
                let right_peg_contain = self.pegs.contains(&right_peg);

                if left_peg_contain ^ right_peg_contain {
                    if left_peg_contain {
                        self.apply_moves((i, j), left_peg, right_peg, &mut outcome_list);
                    } else {
                        self.apply_moves((i, j), right_peg, left_peg, &mut outcome_list);
                    }
                }
            }

            let upper_peg = (*i - 1, *j);
            let bottom_peg = (*i + 1, *j);
            if !Board::is_out_of_bounds(upper_peg) && !Board::is_out_of_bounds(bottom_peg) {
                let is_upper_peg = self.pegs.contains(&upper_peg);
                let is_bottom_peg = self.pegs.contains(&bottom_peg);

                if is_upper_peg ^ is_bottom_peg {
                    if is_upper_peg {
                        self.apply_moves((i, j), upper_peg, bottom_peg, &mut outcome_list)
                    } else {
                        self.apply_moves((i, j), bottom_peg, upper_peg, &mut outcome_list)
                    }
                }
            }
        }

        match method {
            Method::Ordered => (),
            Method::Random => outcome_list.shuffle(&mut thread_rng()),
            Method::Heuristic => outcome_list.sort(),
        }
        for state in outcome_list {
            frontier_list.push_back(state);
        }
    }

    pub fn apply_moves(
        &self,
        (i, j): (&u8, &u8),
        peg_will_murder: (u8, u8),
        peg_will_move_to: (u8, u8),
        outcome_list: &mut Vec<Rc<Board>>,
    ) {
        let mut new_pegs = self.pegs.clone();
        let new_depth = self.depth + 1;
        new_pegs.remove(&(*i, *j));
        new_pegs.remove(&peg_will_murder);
        new_pegs.insert(peg_will_move_to);
        let heuristic_value = Board::calculate_heuristic_value(&new_pegs);
        outcome_list.push(Rc::new(Board {
            pegs: new_pegs,
            depth: new_depth,
            parent: Some(Rc::new(self.clone())),
            heuristic_value,
        }))
    }

    pub fn calculate_heuristic_value(pegs: &BTreeSet<(u8, u8)>) -> u8 {
        let mut result = 0;
        for (i, j) in pegs.iter() {
            result += if !pegs.contains(&(*i + 1, *j)) { 1 } else { 0 };
            result += if !pegs.contains(&(*i - 1, *j)) { 1 } else { 0 };
            result += if !pegs.contains(&(*i, *j + 1)) { 1 } else { 0 };
            result += if !pegs.contains(&(*i, *j - 1)) { 1 } else { 0 };
        }

        result
    }

    pub fn is_out_of_bounds((i, j): (u8, u8)) -> bool {
        return (i < 2 || i > 4) && (j < 2 || j > 4) || i > 6 || j > 6;
    }

    pub fn is_goal_state(&self) -> bool {
        if self.pegs.len() != 1 || self.pegs.get(&(3, 3)).is_none() {
            return false;
        }

        true
    }

    pub fn print_board(&self, iteration_count: u64, depth: u8, clear: bool) {
        let mut board: Vec<Vec<&str>> = vec![vec!["  "; 7]; 7];
        println!("{} {}", iteration_count, depth);
        for i in self.pegs.iter() {
            board[i.0 as usize][i.1 as usize] = "o "
        }

        for i in board.iter().enumerate() {
            for j in i.1.iter().enumerate() {
                if Board::is_out_of_bounds((i.0 as u8, j.0 as u8)) {
                    print!("- ");
                } else {
                    print!("{}", j.1);
                }
            }
            print!("\n");
        }
        if clear {
            print!("\x1b[8F");
        }
    }
}