mod ttt_board;

use ttt_board::{Mark, Board};
use crate::Mark::N;
use crate::Mark::O;
use crate::Mark::X;

fn main() {

    println!("start");

    let mut board_1 = Board {
        board: [[X, O, O], [X, O, X], [N, N, N]]
    };

    let mut board_2 = Board {
        board: [[O, O, X], [O, X, X], [N, X, N]]
    };

    board_1.print_board();
    let best_move1 = board_1.find_best_move(true);
    println!("The best move for X on this board is: row - {}, column - {}", best_move1.0, best_move1.1);

    board_2.print_board();
    let best_move2 = board_2.find_best_move(false);
    println!("The best move for O on this board is: row - {}, column - {}", best_move2.0, best_move2.1);

    println!("end");
}