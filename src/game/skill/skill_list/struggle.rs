use colorful::{Color, Colorful};

use crate::game::{unit::{Unit, Id, Dir}, skill::helper, board::Board};

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

    pub fn exe(&self, board : &mut Board, id : Id) {
        let actor = board.get_unit(id);
        for it in actor.get_catched_with() {
            let actor = board.get_unit(id);
            let target = board.get_unit(it);
            let hit = self.hit(actor, target);
            let is_hit = if hit == 100 {
                println!("[尝试挣扎] {}", "成功".to_string().color(Color::Green));
                true
            }else if hit == 0 {
                println!("[尝试挣扎] {}", "失败".to_string().color(Color::Red));
                false
            }else {
                println!("[尝试挣扎]");
                let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
                helper::show_hit(hit, is_hit, hit_dice, "成功率", "成功", "失败");
                is_hit
            };
            if is_hit {
                board.get_unit_mut(id).cancel_catched_with(it);
                board.get_unit_mut(it).cancel_catch_with(id);
            }
        }
    }
}