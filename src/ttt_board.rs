

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Mark {
    N,
    X,
    O
}

pub struct Board {
    pub board: [Mark; 9]
}

impl Board {
    pub fn print_board(&self) {
        for (i, variant) in self.board.iter().enumerate() {
            println!("Element {}: {:?}", i, variant);
        }
    }
    pub fn board_full(&self) -> bool {
        for element in &self.board {
            if *element == Mark::N {
                return false;
            }
        }
        return true;
    }
}