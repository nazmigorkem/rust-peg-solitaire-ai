mod solitaire;

use self::solitaire::Board;

fn main() {
    let board = Board::new();

    board.generate_possible_moves().iter().for_each(|x| {
        x.generate_possible_moves().iter().for_each(|y| {
            y.generate_possible_moves()
                .iter()
                .for_each(|z| println!("{}", z.generate_possible_moves().len()))
        })
    })
}
