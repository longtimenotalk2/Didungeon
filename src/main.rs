use crate::{game::{board::Board, skill::SkillSet}, wyrand::Dice};

pub mod game;
pub mod wyrand;

fn main() {
    let seed = 114517;
    let mut dice = Dice::new(seed);
    let skill_set = SkillSet::new();
    println!("seed - {seed}");
    let mut board = Board::new_team(&skill_set);
    board.anto_run(30, &mut dice);

}
