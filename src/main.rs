mod game;
extern crate ncurses;

use ncurses::*;

static BOARD_SIZE : usize = 16;

fn main() {
    let mut game = game::Game::new(load_board());

    initscr();
    loop {
        printw(&(game.draw()));
        refresh();
        game.advance();
        std::thread::sleep_ms(1000);
    }
    endwin();
}

fn load_board() -> Vec<Vec<bool>> {
    let mut board : Vec<Vec<bool>> = Vec::with_capacity(BOARD_SIZE);

    //Setup base board.
    for _ in 0..BOARD_SIZE {
        let mut my_vec : Vec<bool> = Vec::with_capacity(BOARD_SIZE);

        for _ in 0..BOARD_SIZE {
            my_vec.push(false);
        }

        board.push(my_vec);
    }

    //Setup a begining state.
    board[1][0] = true;
    board[2][1] = true;
    board[0][2] = true;
    board[1][2] = true;
    board[2][2] = true;

    return board;
}
