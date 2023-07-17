use std::collections::HashMap;

use crate::{game::unit::Unit, wyrand::Dice};

use super::Board;

impl Board {
    pub fn new_noal_vs_kuinuo(seed : u64) -> Self {
        let mut board = Self::new(seed, 2);
        board.insert_unit(Unit::new_noal(0, 0));
        board.insert_unit(Unit::new_kuinuo(1, 1));
        board
    }

    pub fn new(seed : u64, length : i32) -> Self {
        Self {
            indexs: HashMap::new(),
            units: vec!(),
            dice: Dice::new(seed),
            pos_min: 0,
            pos_length: length,
            turn : 1,
            temp_actor_now: None,
            temp_target_now : None,
            temp_remained_bound_value: 0,
        }
    }

    fn insert_unit(&mut self, unit : Unit) {
        let id = unit.get_id();
        self.units.push(unit);
        let index_now = self.units.len() - 1;
        self.indexs.insert(id, index_now);
    }
}