use super::Board;

pub trait DFSWithHeuristic {
    fn solve_dfs_with_heuristic(&self);
    fn calculate_heuristic_value(&self) -> u8;
    // fn generate_possible_moves_with_heuristic(&self) -> Vec<Board>;
}
pub trait DFS {
    fn solve_dfs(&self, is_random: bool);
}

pub trait IterativeDeepining {
    fn solve_iterative_deepening(&self);
}

// pub trait BFS {
//     fn solve_bfs(&self);
// }

// impl Ord for Board {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.calculate_heuristic_value()
//             .cmp(&other.calculate_heuristic_value())
//     }
// }

// impl PartialOrd for Board {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for Board {
//     fn eq(&self, other: &Self) -> bool {
//         self.calculate_heuristic_value() == other.calculate_heuristic_value()
//     }
// }

// impl Eq for Board {}
