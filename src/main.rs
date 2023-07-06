use crate::{game::{board::Board, skill::{SkillSet, struggle::Struggle}}, wyrand::Dice};

pub mod game;
pub mod wyrand;

fn main() {
    let seed = 114517;
    let mut dice = Dice::new(seed);
    let skill_set = SkillSet::new();
    let auto_stand = Struggle::new_auto();
    println!("seed - {seed}");
    let mut board = Board::new_team_8(&skill_set, &auto_stand);
    board.anto_run(100, &mut dice);

}
