use std::{
    cell::RefCell,
    collections::{BTreeSet, LinkedList},
    rc::Rc,
    time::Duration,
};

use rand::{seq::SliceRandom, thread_rng};

use crate::algorithm::enums::Method;

#[derive(Debug, Clone)]
pub struct Board {
    pub pegs: BTreeSet<(u8, u8)>,
    pub depth: u8,
    pub heuristic_value: u8,
    pub parent: Option<Rc<Self>>,
    pub is_solution: RefCell<bool>,
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
            is_solution: RefCell::new(false),
        }
    }

    pub fn generate_possible_moves(
        &self,
        method: &Method,
        frontier_list: &mut LinkedList<Rc<Board>>,
    ) {
        let mut outcome_list: Vec<Rc<Board>> = Vec::new();
        for (i, j) in self.pegs.iter().rev() {
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
        }

        *self.is_solution.borrow_mut() = outcome_list.len() == 0;
        if *self.is_solution.borrow() {
            return;
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
            is_solution: RefCell::new(false),
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

    pub fn print_board(
        &self,
        iteration_count: u64,
        depth: u8,
        clear: bool,
        elapsed_time: Duration,
        memory_usage: u64,
        frontier_list_max_size: usize,
        print_is_solution: bool,
    ) {
        let mut line_count = 0;
        let mut board: Vec<Vec<&str>> = vec![vec!["  "; 7]; 7];
        if iteration_count != 0 {
            line_count += 1;
            println!("\x1B[2KNodes Expanded: {:>12}", iteration_count);
        }
        if !elapsed_time.is_zero() {
            line_count += 1;
            println!(
                "\x1B[2KElapsed Time: {:>13.3?}s",
                elapsed_time.as_secs_f32()
            );
        }
        if memory_usage != 0 {
            line_count += 1;
            println!(
                "\x1B[2KMemory Usage: {:>11.3?} MB",
                (memory_usage as f64 / (1024. * 1024.))
            );
        }
        if frontier_list_max_size != 0 {
            line_count += 1;
            println!("\x1B[2KMaximum Node Count: {:>8}", frontier_list_max_size);
        }
        if depth != 0 {
            line_count += 1;
            println!("\x1B[2KRemaining Pegs: {:>12}", 32 - depth);
        }
        if print_is_solution {
            line_count += 1;
            println!("\x1B[2KIs solution?: {:>14}", self.is_solution.borrow());
        }

        line_count += 7;
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
            print!("\x1b[{line_count}F");
        }
    }
}

impl Ord for Board {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heuristic_value.cmp(&self.heuristic_value)
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic_value == other.heuristic_value
    }
}

impl Eq for Board {}
