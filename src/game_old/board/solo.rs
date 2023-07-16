use crate::{wyrand::Dice, game::skill::Skill};

use super::Board;

impl<'a> Board<'a> {
    pub fn new_solo(skill_set : &'a SkillSet) -> Self {
        Self {
            turn : 0,
            units : vec!(Unit::test_new1(), Unit::test_new2()),
            skill_set,
        }
    }
}