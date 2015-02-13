#![feature(core)]
mod game;

static BOARD_SIZE : usize = 16;

fn main() {
    let game = game::Game::new(load_board());
    game.draw();
    game.advance();
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
    board[1][1] = true;

    return board;
}
