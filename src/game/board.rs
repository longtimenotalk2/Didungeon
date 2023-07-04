mod show;
// mod solo;
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

    fn can(&self, skill : &Skill, ia : u8, ib : Option<u8>) -> bool {
        let skill_can = self.skill_set.get_can(skill);
        skill_can(&self, ia, ib)
    }

    fn exe(&mut self, skill : &Skill, ia : u8, ib : Option<u8>, dice : &mut Dice) -> String {
        let mut skill_exe = self.skill_set.get_exe(skill);
        skill_exe(self, ia, ib, dice)
    }

    fn turn_pass(&mut self) {
        self.turn += 1;
        for unit in &mut self.units {
            unit.action = true;
        }
        let res0 = self.index(0).restore_amount();
        let res1 = self.index(1).restore_amount();
        println!("[End turn] 0 restore {res0}; 1 restore {res1}");
        self.index_mut(0).auto_restore();
        self.index_mut(1).auto_restore();
    }
}