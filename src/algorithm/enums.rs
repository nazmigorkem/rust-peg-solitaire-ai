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
