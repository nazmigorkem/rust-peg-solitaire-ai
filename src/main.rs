mod solitaire;

use self::solitaire::Board;

fn main() {
    let board = Board::new();

    board.generate_possible_moves().iter().for_each(|x| {
        x.print_board();
        x.generate_possible_moves().iter().for_each(|y| {
            y.print_board();
            y.generate_possible_moves()
                .iter()
                .for_each(|z| z.print_board())
        });
        println!("--------");
    })
}
