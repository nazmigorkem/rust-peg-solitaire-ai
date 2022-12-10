use crate::peg_solitaire::Board;

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
    fn solve(
        &self,
        frontier_type: FrontierType,
        method: Method,
        initial_depth_limit: u8,
        time_limit: u16,
    );
    fn get_full_name_of_algorithm(abbreviation: &str) -> &str {
        match abbreviation {
            "bfs" => "Breadth-First Search",
            "dfs" => "Depth-First Search",
            "iterative-dfs" => "Itrative Deepening Search",
            "random-dfs" => "Depth-First Search with Random Selection",
            "heuristic-dfs" => "Depth-First Search with a Node Selection Heuristic",
            _ => "Not Found",
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
