#![feature(core)]
#![feature(std_misc)]
#![feature(io)]

mod game;

use std::old_io::timer;
use std::time::Duration;

static BOARD_SIZE : usize = 16;

fn main() {
    let mut game = game::Game::new(load_board());
    loop {
        game.draw();
        game.advance();
        timer::sleep(Duration::milliseconds(1000));
    }
}

fn load_board() -> Vec<Vec<bool>> {
    let mut board : Vec<Vec<bool>> = Vec::with_capacity(BOARD_SIZE);

    //Setup base board.
    for x in 0..BOARD_SIZE {
        let mut my_vec : Vec<bool> = Vec::with_capacity(BOARD_SIZE);

        for y in 0..BOARD_SIZE {
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
