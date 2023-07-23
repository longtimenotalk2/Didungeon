use colorful::{Color, Colorful};

use crate::game::unit::Id;

use super::Board;

use std::fmt::Write;

impl Board {
    pub fn next_turn(&mut self) {
        self.turn += 1;
        for unit in &mut self.units {
            unit.end_turn()
        }
    }

    pub fn is_ally_win(&self) -> Option<bool> {
        let mut remain_ally = 0;
        let mut remain_enemy = 0;
        for unit in &self.units {
            if !unit.is_defeated() {
                match unit.get_ally() {
                    true => remain_ally += 1,
                    false => remain_enemy += 1,
                }
            }
        }
        if remain_enemy == 0{
            Some(true)
        }else if remain_ally == 0{
            Some(false)
        }else{
            None
        }
    }

    pub fn check_awake(&mut self, s : &mut String) {
        for actor in &mut self.units {
            if actor.check_awake() {
                writeln!(s, "{} {}!", actor.identity(), "苏醒".to_string().color(Color::Yellow)).unwrap();
            }
        }
    }

    pub fn cancel_catch(&mut self, id : Id) -> bool {
        if let Some(it) = self.get_unit(id).get_catch() {
            self.get_unit_mut(id).cancel_catch_with(it);
            self.get_unit_mut(it).cancel_catched_with(id);
            true
        }else{
            false
        }
    }

    pub fn take_stun(&mut self, id : Id) {
        self.get_unit_mut(id).take_stun();
        self.cancel_catch(id);
    }

    
}