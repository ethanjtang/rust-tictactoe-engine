use std::cmp::max;
use std::cmp::min;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Mark 
{
    N,
    X,
    O
}

pub struct Board 
{
    pub board: [[Mark; 3]; 3]
}

impl Board {
    pub fn print_board(&self) 
    {
        for row in 0..3
        {
            for column in 0..3
            {
                if column == 2
                {
                    print!("{:?}\n", self.board[row][column]);
                }
                else
                {
                    print!("{:?} | ", self.board[row][column]);
                }
                
            }    
        }

        print!("\n");
    }
    pub fn is_board_full(&self) -> bool 
    {
        for row in 0..3
        {
            for column in 0..3
            {
                if self.board[row][column] == Mark::N
                {
                    return false;
                }
            }    
        }

        return true;
    }

    pub fn add_mark(&mut self, row: usize, column: usize, mark: Mark) -> bool
    {
        let mut mark_added = false;

        if row > 2 || row < 0 || column < 0 || column > 2
        {
            return false;
        }
        
        if self.board[row][column] == Mark::N
        {
            self.board[row][column] = mark;
            mark_added = true;
        }

        return mark_added;
    }

    pub fn evaluate_board(&self) -> (bool, isize)
    {
        let mut valid_board = false;
        let mut num_scores = 0;
        let mut eval = 0;
        
        // X is pos, O is neg
        
        // Checking horizontal
        for row in 0..3
        {
            if self.board[row][0] == Mark::X && self.board[row][0] == self.board[row][1] && self.board[row][1] == self.board[row][2]
            {
                num_scores += 1;
                eval += 10;
            }

            if self.board[row][0] == Mark::O && self.board[row][0] == self.board[row][1] && self.board[row][1] == self.board[row][2]
            {
                num_scores += 1;
                eval -= 10;
            }
        }

        // Checking vertical
        for column in 0..3
        {
            if self.board[0][column] == Mark::X && self.board[0][column] == self.board[1][column] && self.board[1][column] == self.board[2][column]
            {
                num_scores += 1;
                eval += 10;
            }

            if self.board[0][column] == Mark::O && self.board[0][column] == self.board[1][column] && self.board[1][column] == self.board[2][column]
            {
                num_scores += 1;
                eval -= 10;
            }
        }

        // Checking diagonals

        // neg slope diag
        if self.board[0][0] == Mark::X && self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]
        {
            num_scores += 1;
            eval += 10;
        }

        if self.board[0][0] == Mark::O && self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]
        {
            num_scores += 1;
            eval -= 10;
        }

        // second diag
        if self.board[2][0] == Mark::X && self.board[2][0] == self.board[1][1] && self.board[1][1] == self.board[0][2]
        {
            num_scores += 1;
            eval += 10;
        }

        if self.board[2][0] == Mark::O && self.board[2][0] == self.board[1][1] && self.board[1][1] == self.board[0][2]
        {
            num_scores += 1;
            eval -= 10;
        }


        if num_scores >= 0 && num_scores <= 1
        {
            valid_board = true;
        }

        return (valid_board, eval);
    }  

    pub fn minimax(&mut self, depth: usize, is_max: bool) -> isize
    {
        // We are assuming board is always valid
        let score = self.evaluate_board().1;
        let mut best_score = 0;

        if score == 10 || score == -10
        {
            return score;
        }

        if self.is_board_full() == true
        {
            return 0;
        }

        if is_max
        {
            best_score = -1000;

            for row in 0..3
            {
                for column in 0..3
                {
                    if self.board[row][column] == Mark::N
                    {
                        self.add_mark(row, column, Mark::X);
                        best_score = max(best_score, self.minimax(depth+1, true));
                        self.add_mark(row, column, Mark::N);
                    }
                }
            }

            return best_score;
        }
        else
        {
            best_score = 1000;

            for row in 0..3
            {
                for column in 0..3
                {
                    if self.board[row][column] == Mark::N
                    {
                        self.add_mark(row, column, Mark::O);
                        best_score = min(best_score, self.minimax(depth+1, false));
                        self.add_mark(row, column, Mark::N);
                    }
                }
            }

            return best_score;
        }
    }

    pub fn find_best_move(&mut self, is_max: bool) -> (usize, usize) // row, column
    // assume max is the player
    {
        let mut best_score = 0;
        let mut opt_row = 10;
        let mut opt_column = 10;

        if is_max
        {
            best_score = -1000;
        }
        else
        {
            best_score = 1000;
        }

        for row in 0..3
        {
            for column in 0..3
            {
                if (self.board[row][column] == Mark::N)
                {
                    if is_max
                    {
                        self.add_mark(row, column, Mark::X);
                        let mut move_val = self.minimax(0, false);
                        if move_val > best_score
                        { 
                            opt_row = row;
                            opt_column = column;
                            best_score = move_val;
                        }
                    }
                    else
                    {
                        self.add_mark(row, column, Mark::O);
                        let mut move_val = self.minimax(0, true);
                        if move_val < best_score
                        { 
                            opt_row = row;
                            opt_column = column;
                            best_score = move_val;
                        }
                    }
                }
            }
        }

        return (opt_row, opt_column);
    }
}