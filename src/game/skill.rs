use super::{board::Board, unit::{Id, Dir}};

pub enum Skill {
    Punch
}

pub trait Skillize {
    fn target(&self, board : &Board, ia : Id) -> Vec<(Id, Dir)>;
    fn exe(&self, board : &mut Board, ia : Id, ib : Id, dir : Dir);
}