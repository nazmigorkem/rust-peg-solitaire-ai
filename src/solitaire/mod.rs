use std::collections::{HashSet, VecDeque};

use rand::{seq::SliceRandom, thread_rng};

pub mod enums;
#[derive(Debug, Clone)]
pub struct Board {
    pub pegs: HashSet<(u8, u8)>,
    pub empty_holes: Vec<(u8, u8)>,
    pub depth: usize,
}

impl Board {
    pub fn new() -> Board {
        let mut pegs: HashSet<(u8, u8)> = HashSet::new();
        let mut empty_holes: Vec<(u8, u8)> = Vec::new();
        for i in 0..7 {
            for j in 0..7 {
                if i == 3 && j == 3 {
                    empty_holes.push((i, j));
                } else if !((i < 2 || i > 4) && (j < 2 || j > 4)) {
                    pegs.insert((i, j));
                }
            }
        }
        Board {
            pegs,
            empty_holes,
            depth: 0,
        }
    }

    pub fn generate_possible_moves(&self, is_random: bool, frontier_list: &mut VecDeque<Board>) {
        let current_empty_holes = &self.empty_holes;
        let mut indexes: Vec<usize> = (0..current_empty_holes.len()).collect();
        if is_random {
            indexes.shuffle(&mut thread_rng());
        }
        for index in indexes {
            let (i, j) = current_empty_holes[index];
            for k in vec![-2, 2] {
                let mut direction: bool = true;
                let mut is_ok = false;
                if i as i16 + k >= 0
                    && self.pegs.contains(&((i as i16 + k) as u8, j))
                    && self.pegs.contains(&((i as i16 + k / 2) as u8, j))
                {
                    direction = true;
                    is_ok = true;
                }
                if j as i16 + k >= 0
                    && self.pegs.contains(&(i, (j as i16 + k) as u8))
                    && self.pegs.contains(&(i, (j as i16 + k / 2) as u8))
                {
                    is_ok = true;
                    direction = false;
                }
                if is_ok {
                    frontier_list.push_back(Board::apply_moves(
                        self.pegs.clone(),
                        self.empty_holes.clone(),
                        i as i16,
                        j as i16,
                        k,
                        direction,
                        self.depth + 1,
                        index,
                    ));
                }
            }
        }
    }

    pub fn apply_moves(
        pegs: HashSet<(u8, u8)>,
        empty_holes: Vec<(u8, u8)>,
        i: i16,
        j: i16,
        direction: i16,
        is_vertical: bool,
        new_depth: usize,
        empty_peg_index: usize,
    ) -> Board {
        let mut new_pegs = pegs;
        let mut new_empty_holes = empty_holes;
        let died_peg_position: (u8, u8);
        let murderer_peg_position: (u8, u8);
        if is_vertical {
            died_peg_position = ((i + direction / 2) as u8, j as u8);
            murderer_peg_position = ((i + direction) as u8, j as u8);
        } else {
            died_peg_position = (i as u8, (j + direction / 2) as u8);
            murderer_peg_position = (i as u8, (j + direction) as u8);
        }
        new_pegs.remove(&died_peg_position);
        new_pegs.remove(&murderer_peg_position);
        new_empty_holes.remove(empty_peg_index);

        new_empty_holes.push(died_peg_position);
        new_empty_holes.push(murderer_peg_position);
        new_pegs.insert((i as u8, j as u8));

        return Board {
            pegs: new_pegs,
            empty_holes: new_empty_holes,
            depth: new_depth,
        };
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
        let mut board: Vec<Vec<&str>> = vec![vec!["- "; 7]; 7];
        println!("{} {}", iteration_count, depth);
        for i in self.pegs.iter() {
            board[i.0 as usize][i.1 as usize] = "o ";
        }
        for i in self.empty_holes.iter() {
            board[i.0 as usize][i.1 as usize] = "  ";
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
