mod algorithm;
mod solitaire;

use algorithm::dfs::DFS;

use self::solitaire::Board;

fn main() {
    let board = Board::new();

    println!("{}", board.solve_dfs());
}
