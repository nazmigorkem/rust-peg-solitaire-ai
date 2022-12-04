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
                if !(i == 3 && j == 3) && !((i < 2 || i > 4) && (j < 2 || j > 4)) {
                    pegs.insert((i, j));
                }
            }
        }
        Board { pegs, depth: 0 }
    }

    pub fn generate_possible_moves(&self, is_random: bool, frontier_list: &mut VecDeque<Board>) {}

    pub fn apply_moves() {}

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
