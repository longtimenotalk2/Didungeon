mod show;
mod solo;
// mod skill;

use crate::wyrand::Dice;
use super::{unit::Unit, skill::{Skill, SkillSet}};

pub struct Board<'a> {
    turn : i32,
    units : Vec<Unit>,
    skill_set : &'a SkillSet,
}

impl<'a> Board<'a> {
    pub fn new_solo(skill_set : &'a SkillSet) -> Self {
        Self {
            turn : 0,
            units : vec!(Unit::test_new1(), Unit::test_new2()),
            skill_set,
        }
    }

    pub fn index(&self, id : u8) -> &Unit {
        let i : usize = id.try_into().unwrap();
        self.units.get(i).unwrap()
    }

    pub fn index_mut(&mut self, id : u8) -> &mut Unit {
        let i : usize = id.try_into().unwrap();
        self.units.get_mut(i).unwrap()
    }

    fn can(&self, skill : &Skill, ia : u8, ib : u8) -> bool {
        self.skill_set.can(skill, self, ia, ib)
    }

    fn evaluate(&self, skill : &Skill, ia : u8, ib: u8) -> (i32, Option<String>) {
        self.skill_set.evaluate(skill, self, ia, ib)
    }

    fn exe(&mut self, skill : &Skill, ia : u8, ib : u8, dice : &mut Dice) -> String {
        self.skill_set.exe(skill, self, ia, ib, dice)
    }

    fn turn_pass(&mut self) {
        self.turn += 1;
        for unit in &mut self.units {
            unit.end_turn();
        }
    }
}