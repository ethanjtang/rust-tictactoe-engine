

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
    pub board: [Mark; 9]
}

impl Board {
    pub fn print_board(&self) 
    {
        for n in 1..10
        {
            if n % 3 == 0
            {
                print!("{:?}\n", self.board[n-1]);
            }
            else
            {
                print!("{:?} | ", self.board[n-1]);
            }
        }
    }
    pub fn is_board_full(&self) -> bool 
    {
        for n in 1..10
        {
            if self.board[n-1] == Mark::N
            {
                return false;
            }
        }
        return true;
    }
}