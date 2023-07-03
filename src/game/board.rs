mod show;
mod solo;
mod skill;

use crate::wyrand::Dice;
use super::unit::Unit;

pub struct Board {
    turn : i32,
    units : Vec<Unit>,
    dice : Dice,
}

impl Board {
    pub fn new_solo(seed : u64) -> Self {
        Self {
            turn : 0,
            units : vec!(Unit::test_new1(), Unit::test_new2()),
            dice : Dice::new(seed),
        }
    }

    fn index(&self, id : i32) -> &Unit {
        let i : usize = id.try_into().unwrap();
        self.units.get(i).unwrap()
    }

    fn index_mut(&mut self, id : i32) -> &mut Unit {
        let i : usize = id.try_into().unwrap();
        self.units.get_mut(i).unwrap()
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