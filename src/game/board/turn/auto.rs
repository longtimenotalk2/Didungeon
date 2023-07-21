use crate::game::{board::{Board, Phase}, unit::Id, skill::skill_list::{struggle::Struggle, force_unbound::ForceUnbound}};

use super::Return;

use std::fmt::Write;

impl Board {
    pub fn turn_auto(&mut self, need_show : bool, id : Id) -> Return {
        let mut str = String::new();
        let s = &mut str;

        // 解除眩晕
        if self.get_unit(id).is_stun() {
            self.get_unit_mut(id).recover_stun();
            write!(s, "从眩晕中恢复\n").unwrap();
        }

        // 自动挣扎
        if self.get_unit(id).is_catched() {
            Struggle::new().exe(s, self, id);
        }

        // 自动挣脱
        if self.get_unit(id).has_bound() {
            ForceUnbound::new().exe(s, self, id)
        }

        // 自动起身
        if self.get_unit(id).is_fall() {
            // [起身]
            if self.get_unit_mut(id).check_to_stand() {
                write!(s, "[起身]\n").unwrap();
            } 
        }

        self.phase = Phase::Main {id};
        self.string_cache += &str;

        self.continue_turn(need_show)
    }
}