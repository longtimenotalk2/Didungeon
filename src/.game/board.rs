mod show;
mod run;
mod new;
mod action;

use std::collections::HashMap;

use crate::wyrand::Dice;
use super::{skill::{Skill, SkillSet, struggle::Struggle}, unit::Unit};

pub struct Board<'a> {
    turn : i32,
    units : Vec<Unit>,
    locations : HashMap<u8, i32>,
    skill_set : &'a SkillSet,
    anto_stand : &'a Struggle,
}

impl<'a> Board<'a> {
    pub fn index(&self, id : u8) -> &Unit {
        let i : usize = id.try_into().unwrap();
        self.units.get(i).unwrap()
    }

    pub fn index_mut(&mut self, id : u8) -> &mut Unit {
        let i : usize = id.try_into().unwrap();
        self.units.get_mut(i).unwrap()
    }

    fn target(&self, skill : &Skill, ia : u8) -> Vec<u8> {
        self.skill_set.target(skill, self, ia)
    }

    fn evaluate(&self, skill : &Skill, ia : u8, ib: u8) -> (i32, Option<String>) {
        self.skill_set.evaluate(skill, self, ia, ib)
    }

    fn exe(&mut self, skill : &Skill, ia : u8, ib : u8, dice : &mut Dice) -> String {
        self.skill_set.exe(skill, self, ia, ib, dice)
    }

    
}