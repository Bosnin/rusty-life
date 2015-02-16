pub struct Game {
    board: Vec<Vec<bool>>,
}

impl Game {
    pub fn new(board: Vec<Vec<bool>>) -> Game {
        return Game{ board:board };
    }

    pub fn draw(&self) {
        print!("\n");
        for row in self.board.iter() {
            print!("| ");
            for place in row.iter() {
                let spot = if *place { "* " } else { "  " };
                print!("{}", spot);
            }
            println!("|");
        }
        print!("\n");
    }

    pub fn advance(&mut self) {
        let mut new_board = self.board.clone();
        for x in 0..self.board.len() {
            for y in 0..self.board[x].len() {
                let neighbors = self.count_neighbors(x,y);
                if self.board[x][y] {
                    if neighbors < 2 {
                        new_board[x][y] = false;
                    } else if neighbors > 3 {
                        new_board[x][y] = false;
                    }
                } else {
                    if neighbors == 3 {
                        new_board[x][y] == true;
                    }
                }
            }
        }
        self.board = new_board.clone();
    }

    fn count_neighbors(&self, x: usize, y: usize) -> i64 {
        let mut sum: i64 = 0;
        let modu = self.board.len();
        if self.board[(x-1)%modu][y] {
            sum = sum + 1;
        }
        if self.board[(x+1)%modu][y] {
            sum = sum + 1;
        }
        if self.board[x][(y-1)%modu] {
            sum = sum + 1;
        }
        if self.board[x][(y+1)%modu] {
            sum = sum + 1;
        }
        if self.board[(x-1)%modu][(y-1)%modu] {
            sum = sum + 1;
        }
        if self.board[(x-1)%modu][(y+1)%modu] {
            sum = sum + 1;
        }
        if self.board[(x+1)%modu][(y-1)%modu] {
            sum = sum + 1;
        }
        if self.board[(x+1)%modu][(y+1)%modu] {
            sum = sum + 1;
        }
        return sum;
    }
}
