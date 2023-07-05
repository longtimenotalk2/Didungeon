use crate::game::{skill::SkillSet, unit::Unit};

use super::Board;

impl<'a> Board<'a> {
    pub fn new_team(skill_set : &'a SkillSet) -> Self {
        let mut board = Self {
            turn : 0,
            units : vec!(),
            skill_set,
        };

        board.units.push(Unit::new_blank(true));
        board.units.push(Unit::new_blank(true));
        board.units.push(Unit::new_blank(false));
        board.units.push(Unit::new_blank(false));

        board
    }
}