pub struct Game {
    board: Vec<Vec<bool>>,
}

impl Game {
    pub fn new(board: Vec<Vec<bool>>) -> Game {
        return Game{ board:board };
    }

    pub fn draw(&self) {
        for row in self.board.iter() {
            print!("| ");
            for place in row.iter() {
                let spot = if *place { "* " } else { "  " };
                print!("{}", spot);
            }
            println!("|");
        }
    }

    pub fn advance(&self) {
        println!("Not yet");
    }
}
