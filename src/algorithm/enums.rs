use crate::solitaire::Board;

#[derive(PartialEq)]
pub enum Method {
    Random,
    Ordered,
    Heuristic,
}

#[derive(PartialEq)]
pub enum FrontierType {
    Stack,
    Queue,
}

pub trait Algorithm {
    fn solve(&self, frontier_type: FrontierType, method: Method, initial_depth_limit: u8);
}

pub trait BFS {
    fn solve_bfs(&self);
}

impl Ord for Board {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.calculate_heuristic_value()
            .cmp(&other.calculate_heuristic_value())
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.calculate_heuristic_value() == other.calculate_heuristic_value()
    }
}

impl Eq for Board {}
