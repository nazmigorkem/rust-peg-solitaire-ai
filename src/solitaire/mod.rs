use std::{
    cell::{Ref, RefCell},
    collections::{BTreeSet, VecDeque},
    rc::Rc,
};

pub mod enums;
#[derive(Debug, Clone)]
pub struct Board {
    pub pegs: BTreeSet<(u8, u8)>,
    pub depth: usize,
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
        Board {
            pegs,
            depth: 0,
            parent: None,
        }
    }

    pub fn generate_possible_moves(
        &self,
        is_random: bool,
        frontier_list: &mut VecDeque<Rc<Board>>,
    ) {
        for (i, j) in self.pegs.iter().rev() {
            let upper_peg = (*i - 1, *j);
            let bottom_peg = (*i + 1, *j);
            if !Board::is_out_of_bounds(upper_peg) && !Board::is_out_of_bounds(bottom_peg) {
                let is_upper_peg = self.pegs.contains(&upper_peg);
                let is_bottom_peg = self.pegs.contains(&bottom_peg);

                if is_upper_peg ^ is_bottom_peg {
                    if is_upper_peg {
                        self.apply_moves((i, j), upper_peg, bottom_peg, frontier_list)
                    } else {
                        self.apply_moves((i, j), bottom_peg, upper_peg, frontier_list)
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
                        self.apply_moves((i, j), left_peg, right_peg, frontier_list);
                    } else {
                        self.apply_moves((i, j), right_peg, left_peg, frontier_list);
                    }
                }
            }
        }
    }

    pub fn apply_moves(
        &self,
        (i, j): (&u8, &u8),
        peg_will_murder: (u8, u8),
        peg_will_move_to: (u8, u8),
        frontier_list: &mut VecDeque<Rc<Board>>,
    ) {
        let mut new_pegs = self.pegs.clone();
        let new_depth = self.depth + 1;
        new_pegs.remove(&(*i, *j));
        new_pegs.remove(&peg_will_murder);
        new_pegs.insert(peg_will_move_to);
        frontier_list.push_back(Rc::new(Board {
            pegs: new_pegs,
            depth: new_depth,
            parent: Some(Rc::new(self.clone())),
        }))
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

    pub fn print_board(&self, iteration_count: u64, depth: usize, clear: bool) {
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
