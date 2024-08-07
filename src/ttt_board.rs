

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
        for i in 0..3
        {
            for j in 0..3
            {
                print!("{:?}", self.board[i][j]);
            }    
        }

        print!("\n");
    }
    pub fn is_board_full(&self) -> bool 
    {
        for i in 0..3
        {
            for j in 0..3
            {
                if self.board[i][j] == Mark::N
                {
                    return false;
                }
            }    
        }

        return true;
    }

    pub fn add_mark(&mut self, pos: usize, mark: Mark) -> bool
    {
        let mut mark_added = false;

        if pos >= 9
        {
            return false;
        }

        let row = pos / 3;
        let column = pos % 3;
        
        if self.board[row][column] == Mark::N
        {
            self.board[row][column] = mark;
            mark_added = true;
        }

        return mark_added;
    }

    pub fn evaluate_board(&self) -> (bool, usize)
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

        // Checking diagonals

        if num_scores >= 0 && num_scores <= 1
        {
            valid_board = true;
        }

        return (valid_board, eval);
    }   
}