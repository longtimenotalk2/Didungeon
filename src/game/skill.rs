pub mod punch;

pub mod helper;

use super::{board::Board, unit::{Id, Dir}};


pub enum Skill {
    Punch
}

impl Skill {
    pub fn name(&self) -> &'static str {
        match self {
            Skill::Punch => "挥拳",
        }
    }
}

pub trait Skillize {
    fn get_targets(&self, board : &Board, id : Id) -> Vec<(Id, Dir)>;
    fn exe(&self, board : &mut Board, id : Id, it : Id, dir : Dir);
}