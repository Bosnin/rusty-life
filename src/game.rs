pub struct Game {
    board: Vec<Vec<bool>>,
}

impl Game {
    pub fn new(board: Vec<Vec<bool>>) -> Game {
        return Game{ board:board };
    }

    pub fn draw(&self) -> String {
        let mut drawn_board = String::new();
        drawn_board = drawn_board + "\n";
        for row in self.board.iter() {
            drawn_board = drawn_board + "| ";
            for place in row.iter() {
                let spot = if *place { "* " } else { "  " };
                drawn_board = drawn_board + spot;
            }
            drawn_board = drawn_board + "|\n";
        }
        drawn_board = drawn_board + "\n";
        return drawn_board;
    }

    pub fn advance(&mut self) {
        let mut new_board = self.board.clone();
        for x in 0..self.board.len() {
            for y in 0..self.board[x].len() {
                let neighbors = self.count_neighbors(x,y);
                //println!("{}, {} : {}", x, y, neighbors);
                if self.board[x][y] {
                    if neighbors < 2 {
                        new_board[x][y] = false;
                    } else if neighbors > 3 {
                        new_board[x][y] = false;
                    } else {
                        new_board[x][y] = true;
                    }
                } else {
                    if neighbors == 3 {
                        new_board[x][y] = true;
                    } else {
                        new_board[x][y] = false;
                    }
                }
            }
        }
        self.board = new_board.clone();
    }

    fn count_neighbors(&self, x: usize, y: usize) -> i64 {
        let mut sum: i64 = 0;
        let modu = self.board.len() - 1;

        if self.board[self.u_add(modu,x,-1)][y] {
            sum = sum + 1;
        }
        if self.board[self.u_add(modu,x,1)][y] {
            sum = sum + 1;
        }
        if self.board[x][self.u_add(modu,y,-1)] {
            sum = sum + 1;
        }
        if self.board[x][self.u_add(modu,y,1)] {
            sum = sum + 1;
        }
        if self.board[self.u_add(modu,x,-1)][self.u_add(modu,y,-1)] {
            sum = sum + 1;
        }
        if self.board[self.u_add(modu,x,-1)][self.u_add(modu,y,1)] {
            sum = sum + 1;
        }
        if self.board[self.u_add(modu,x,1)][self.u_add(modu,y,-1)] {
            sum = sum + 1;
        }
        if self.board[self.u_add(modu,x,1)][self.u_add(modu,y,1)] {
            sum = sum + 1;
        }
        return sum;
    }

    //Had to deal with wrapping usize around my board size
    fn u_add(&self, max: usize, n: usize, m: i64) -> usize {
        let min = 0;
        if m > 0 {
            if n == max {
                return min;
            } else {
                return n + 1
            }
        } else {
            if n == min {
                return max;
            } else {
                return n - 1
            }

        }
    }
}
