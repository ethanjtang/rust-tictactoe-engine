mod ttt_board;

use ttt_board::{Mark, Board};
use crate::Mark::N;
use crate::Mark::O;
use crate::Mark::X;

fn main() {

    println!("start");

    let mut test_board = Board {
        board: [[N, N, N], [N, N, N], [N, N, N]]
    };

    let test_board2 = Board {
        board: [[X, X, X], [N, N, N], [N, N, N]]
    };

    test_board.print_board();
    println!("Board is full: {}", test_board.is_board_full());

    test_board2.print_board();
    println!("Board is full: {}", test_board2.is_board_full());

    println!("Adding mark to empty board");
    let boolean = test_board.add_mark(0, Mark::O);
    println!("Mark was added succuessfully: {boolean}");
    test_board.print_board();

    println!("Test board 2 is valid: {}, eval: {}", test_board2.evaluate_board().0, test_board2.evaluate_board().1);

    println!("end");
}