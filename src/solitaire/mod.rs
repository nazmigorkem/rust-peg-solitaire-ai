use std::collections::HashSet;

pub mod enums;
#[derive(Debug, Clone)]
pub struct Board {
    pub pegs: HashSet<(u8, u8)>,
    pub empty_holes: HashSet<(u8, u8)>,
    pub depth: usize,
}

impl Board {
    pub fn new() -> Board {
        let mut pegs: HashSet<(u8, u8)> = HashSet::new();
        let mut empty_holes: HashSet<(u8, u8)> = HashSet::new();
        for i in 0..7 {
            for j in 0..7 {
                if i == 3 && j == 3 {
                    empty_holes.insert((i, j));
                } else if !(((i < 2 || i > 4) && j < 2) || ((i < 2 || i > 4) && j > 4)) {
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

    pub fn print_board(&self) {
        let mut board: Vec<Vec<&str>> = vec![vec!["- "; 7]; 7];

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

    pub fn generate_possible_moves(&self, _is_random: bool) -> Vec<Board> {
        let mut outcome: Vec<Board> = Vec::new();
        for (i, j) in self.empty_holes.clone() {
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
                    outcome.push(Board::apply_moves(
                        self.pegs.clone(),
                        self.empty_holes.clone(),
                        i as i16,
                        j as i16,
                        k,
                        direction,
                        self.depth + 1,
                    ));
                }
            }
        }
        outcome
    }

    pub fn apply_moves(
        pegs: HashSet<(u8, u8)>,
        empty_holes: HashSet<(u8, u8)>,
        i: i16,
        j: i16,
        direction: i16,
        is_vertical: bool,
        new_depth: usize,
    ) -> Board {
        let mut new_pegs = pegs.clone();
        let mut new_empty_holes = empty_holes.clone();
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
        new_empty_holes.remove(&(i as u8, j as u8));

        new_empty_holes.insert(died_peg_position);
        new_empty_holes.insert(murderer_peg_position);
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
}
