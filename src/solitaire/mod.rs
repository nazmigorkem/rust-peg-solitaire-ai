use std::collections::HashSet;

pub mod enums;
#[derive(Debug, Clone)]
pub struct Board {
    pub pegs: HashSet<(usize, usize)>,
    pub empty_holes: HashSet<(usize, usize)>,
    pub depth: usize,
}

impl Board {
    pub fn new() -> Board {
        let mut pegs: HashSet<(usize, usize)> = HashSet::new();
        let mut empty_holes: HashSet<(usize, usize)> = HashSet::new();
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
            board[i.0][i.1] = "o ";
        }
        for i in self.empty_holes.iter() {
            board[i.0][i.1] = "  ";
        }

        for i in board {
            for j in i {
                print!("{}", j);
            }
            print!("\n");
        }
        print!("\x1b[8F");
    }

    pub fn generate_possible_moves(&self) -> Vec<Board> {
        let mut outcome: Vec<Board> = Vec::new();
        for (i, j) in self.empty_holes.clone() {
            for k in vec![-2, 2] {
                if i as i16 + k >= 0
                    && self.pegs.contains(&((i as i16 + k) as usize, j))
                    && self.pegs.contains(&((i as i16 + k / 2) as usize, j))
                {
                    outcome.push(Board::apply_moves(
                        self.pegs.clone(),
                        self.empty_holes.clone(),
                        i as i16,
                        j as i16,
                        k,
                        true,
                        self.depth + 1,
                    ));
                }
                if j as i16 + k >= 0
                    && self.pegs.contains(&(i, (j as i16 + k) as usize))
                    && self.pegs.contains(&(i, (j as i16 + k / 2) as usize))
                {
                    outcome.push(Board::apply_moves(
                        self.pegs.clone(),
                        self.empty_holes.clone(),
                        i as i16,
                        j as i16,
                        k,
                        false,
                        self.depth + 1,
                    ));
                }
            }
        }
        outcome
    }

    pub fn apply_moves(
        pegs: HashSet<(usize, usize)>,
        empty_holes: HashSet<(usize, usize)>,
        i: i16,
        j: i16,
        direction: i16,
        is_vertical: bool,
        new_depth: usize,
    ) -> Board {
        let mut new_pegs = pegs.clone();
        let mut new_empty_holes = empty_holes.clone();
        if is_vertical {
            let died_peg_position = ((i + direction / 2) as usize, j as usize);
            let murderer_peg_position = ((i + direction) as usize, j as usize);

            new_pegs.remove(&died_peg_position);
            new_pegs.remove(&murderer_peg_position);
            new_empty_holes.remove(&(i as usize, j as usize));

            new_empty_holes.insert(died_peg_position);
            new_empty_holes.insert(murderer_peg_position);
            new_pegs.insert((i as usize, j as usize));

            return Board {
                pegs: new_pegs,
                empty_holes: new_empty_holes,
                depth: new_depth,
            };
        } else {
            let died_peg_position = (i as usize, (j + direction / 2) as usize);
            let murderer_peg_position = (i as usize, (j + direction) as usize);

            new_pegs.remove(&died_peg_position);
            new_pegs.remove(&murderer_peg_position);
            new_empty_holes.remove(&(i as usize, j as usize));

            new_empty_holes.insert(died_peg_position);
            new_empty_holes.insert(murderer_peg_position);
            new_pegs.insert((i as usize, j as usize));

            return Board {
                pegs: new_pegs,
                empty_holes: new_empty_holes,
                depth: new_depth,
            };
        }
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
