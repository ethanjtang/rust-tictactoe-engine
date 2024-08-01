mod ttt_board;

use ttt_board::{Mark, Board};
use crate::Mark::N;
use crate::Mark::O;
use crate::Mark::X;

fn main() {

    println!("start");

    let test_board = Board {
        board: [N, N, N, N, N, N, N, N, N]
    };

    let test_board2 = Board {
        board: [X, X, O, O, X, X, X, X, X]
    };

    test_board.print_board();
    println!("Board is full: {}", test_board.is_board_full());

    test_board2.print_board();
    println!("Board is full: {}", test_board2.is_board_full());

    println!("end");
}