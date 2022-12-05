use std::collections::{BTreeSet, VecDeque};

pub mod enums;
#[derive(Debug, Clone)]
pub struct Board {
    pub pegs: BTreeSet<(u8, u8)>,
    pub depth: usize,
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
        Board { pegs, depth: 0 }
    }

    pub fn generate_possible_moves(&self, is_random: bool, frontier_list: &mut VecDeque<Board>) {
        for (i, j) in self.pegs.iter() {
            for length in [-2, 2] {
                let peg_will_move_to = ((*i as i16 + length) as u8, *j);
                let peg_will_murder = ((*i as i16 + length / 2) as u8, *j);
                if !Board::is_out_of_bounds(peg_will_move_to)
                    && !self.pegs.contains(&peg_will_move_to) 
                    && self.pegs.contains(&peg_will_murder) {
                    self.apply_moves((i, j), peg_will_murder, peg_will_move_to, frontier_list);
                }
                let peg_will_move_to = (*i, (*j as i16 + length) as u8);
                let peg_will_murder = (*i, (*j as i16 + length / 2) as u8);
                if !Board::is_out_of_bounds(peg_will_move_to) &&
                    !self.pegs.contains(&peg_will_move_to) 
                    && self.pegs.contains(&peg_will_murder) {
                        self.apply_moves((i, j), peg_will_murder, peg_will_move_to, frontier_list);
                }
            }
        }
    }

    pub fn apply_moves(&self, (i, j): (&u8, &u8), peg_will_murder: (u8, u8), peg_will_move_to: (u8, u8), frontier_list: &mut VecDeque<Board>) {
        let mut new_pegs = self.pegs.clone();
                    new_pegs.remove(&(*i, *j));
                    new_pegs.remove(&peg_will_murder);
                    new_pegs.insert(peg_will_move_to);
                    frontier_list.push_back(Board {
                        pegs: new_pegs,
                        depth: self.depth + 1
                    })
    }

    pub fn is_out_of_bounds((i, j): (u8, u8)) -> bool {
        return (i < 2 || i > 4) && (j < 2 || j > 4) || i > 6 || j > 6
    }

    pub fn is_goal_state(&self) -> bool {
        if self.pegs.len() != 1 {
            return false;
        }
        if self.pegs.get(&(3, 3)).is_none() {
            return false;
        }

        true
    }

    pub fn print_board(&self, iteration_count: u64, depth: usize) {
        let mut board: Vec<Vec<&str>> = vec![vec!["  "; 7]; 7];
        println!("{} {}", iteration_count, depth);
        for i in self.pegs.iter() {
            board[i.0 as usize][i.1 as usize] = "o "
        }

        for i in board {
            for j in i {
                print!("{}", j);
            }
            print!("\n");
        }
        print!("\x1b[8F");
    }
}
