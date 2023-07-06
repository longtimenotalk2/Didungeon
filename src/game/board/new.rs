use std::collections::HashMap;

use crate::game::{skill::SkillSet, unit::Unit};

use super::Board;

impl<'a> Board<'a> {
    pub fn new_team(skill_set : &'a SkillSet) -> Self {
        let mut board = Self {
            turn : 0,
            units : vec!(),
            locations : HashMap::new(),
            skill_set,
        };

        board.insert_unit(Unit::new_blank(true));
        board.insert_unit(Unit::new_blank(true));
        board.insert_unit(Unit::new_blank(false));
        board.insert_unit(Unit::new_blank(false));

        board
    }
}

impl<'a> Board<'a> {
    fn insert_unit(&mut self, unit : Unit) {
        let l = match self.locations.values().max() {
            Some(n) => n+1,
            None => 0,
        };
        let i : u8 = self.units.len().try_into().unwrap();
        self.locations.insert(i, l);
        self.units.push(unit);
    }
}