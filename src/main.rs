use crate::game::{board::Board};

pub mod game;
pub mod wyrand;

fn main() {
    let seed = 114516;
    println!("seed - {seed}");
    let mut board = Board::new_solo(seed);
    board.solo_start(10);

}
