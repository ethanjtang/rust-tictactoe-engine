

#[derive(Debug)]
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
}