mod ttt_board;

use ttt_board::{Mark, Board};
use crate::Mark::N;

fn main() {

    println!("Creating board...");

    let test_board = Board {
        board: [N, N, N, N, N, N, N, N, N]
    };

    println!("Board successfully created!");

    println!("Printing board...");

    test_board.print_board();

    println!("Board finished printing!");
}