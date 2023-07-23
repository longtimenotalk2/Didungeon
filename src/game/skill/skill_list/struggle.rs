use colorful::{Color, Colorful};

use crate::game::{unit::{Unit, Id}, skill::helper, board::Board};
use std::fmt::Write;

pub struct Struggle {
    basic_hit : i32,
    hit_rate : i32,
}

impl Struggle {
    pub fn new() -> Self {
        Self {
            basic_hit: 50,
            hit_rate: 5,
        }
    }

    fn hit(&self, actor : &Unit, target : &Unit) -> i32 {
        let acc = actor.struggle_force();
        let evd = target.hold_force();
        if acc == 0 {
            return 0;
        }
        if evd == 0 {
            return 100;
        }
        helper::to_hit(self.basic_hit + self.hit_rate * (acc - evd))
    }

    pub fn exe(&self, s : &mut String, board : &mut Board, id : Id) {
        let actor = board.get_unit(id);

        // [挣扎] (100%成功率) 成功
        // [挣扎] (0%成功率) 失败
        // (67%成功率 → 🎲 : 89) 失败

        for it in actor.get_catched() {
            let actor = board.get_unit(id);
            let target = board.get_unit(it);
            let hit = self.hit(actor, target);
            let is_hit = if hit == 100 {
                write!(s, "[挣扎] (100%成功率) ").unwrap();
                true
            }else if hit == 0 {
                write!(s, "[挣扎] (0%成功率) ").unwrap();
                false
            }else {
                write!(s, "[挣扎] ").unwrap();
                let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
                helper::write_hit_small(s, hit, is_hit, hit_dice.unwrap_or(0));
                write!(s, " ").unwrap();
                is_hit
            };
            match is_hit {
                true => {
                    board.get_unit_mut(id).cancel_catched_with(it);
                    board.get_unit_mut(it).cancel_catch_with(id);
                    write!(s, "{}", "成功".to_string().color(Color::Green)).unwrap();
                },
                false => write!(s, "{}", "失败".to_string().color(Color::Red)).unwrap(),
            }
            write!(s, "\n").unwrap();
        }
    }
}