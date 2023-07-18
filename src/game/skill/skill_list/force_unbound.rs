use std::collections::HashMap;

use colorful::{Color, Colorful};

use crate::game::{board::Board, unit::{Id, bound::BoundPart, Unit}, skill::helper};

pub struct ForceUnbound {
    basic_hit : i32,
    hit_rate : i32,
}

impl ForceUnbound {
    pub fn new() -> Self {
        Self {
            basic_hit: 0,
            hit_rate: 5,
        }
    }

    pub fn hit(&self, actor : &Unit, is_upper : bool) -> i32 {
        let acc = match is_upper {
            true => actor.unbound_force_upper(),
            false => actor.unbound_force_lower(),
        };
        helper::to_hit(self.basic_hit + self.hit_rate * acc)
    }

    pub fn exe(&self, board : &mut Board, id : Id) {
        let mut unbound_hash : HashMap<BoundPart, i32> = HashMap::new();

        let actor = board.get_unit(id);
        if let Some(bound) = actor.next_force_upper() {
            let force = self.hit(actor, true);
            let r = unbound_hash.entry(bound).or_insert(0);
            *r += force
        }
        if let Some(bound) = actor.next_force_lower() {
            let force = self.hit(actor, false);
            let r = unbound_hash.entry(bound).or_insert(0);
            *r += force
        }

        for (bound, hit) in unbound_hash {
            let actor = board.get_unit(id);
            let tight = actor.get_tightness(&bound);
            let hit = (hit + 100 - tight).min(100);
            println!("尝试挣脱 {} {}", actor.bound_identity_change(&bound, false), bound.name());
            let is_hit = if hit == 100{
                println!("挣脱成功率 : 100% -> {}", "成功".to_string().color(Color::Green));
                true
            }else if hit == 0{
                println!("挣脱成功率 : 0% -> {}", "失败".to_string().color(Color::Red));
                false
            }else{
                let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
                helper::show_hit(hit, is_hit, hit_dice, "挣脱成功率", "成功", "失败");
                is_hit
            };
            if is_hit {
                board.get_unit_mut(id).untie(&bound);
            } else {
                let actor = board.get_unit(id);
                let tight = actor.get_tightness(&bound);
                let new_tight = 100 - hit;
                if tight != new_tight {
                    println!("绳索强度 : {} -> {}", tight, new_tight.to_string().color(Color::Green));
                }
                board.get_unit_mut(id).tightness_change_to(&bound, new_tight);
            }
        }
    }
}