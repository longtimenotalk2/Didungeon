use crate::game::{board::Board};

pub mod game;
pub mod wyrand;

fn main() {
    println!("Hello, world!");
    let mut board = Board::new_solo(114514);
    board.solo_start(100);

}
