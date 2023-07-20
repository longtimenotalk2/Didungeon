use std::collections::HashMap;

use colorful::{Color, Colorful};

use crate::game::{board::Board, unit::{Id, bound::BoundPart, Unit}, skill::helper};
use std::fmt::Write;
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

    pub fn exe(&self, s : &mut String, board : &mut Board, id : Id) {

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

            if hit > 0{
                // 挣脱 [...](67%) 手腕 成功 (消耗点数 : 33)
                // 挣脱 [...](50%) 手腕 (30%成功率 → 🎲 : 71) 挣脱至 → 30% (消耗点数 : 67)
                let bound_idy = actor.bound_identity(Some((&bound, false)), false);
                let tight_idy = actor.identity_tightness(&bound);
                let bound_name_idy = bound.name();
                write!(s, "[挣脱] {bound_idy}{tight_idy} {bound_name_idy} ").unwrap();

                let is_hit = if hit == 100{
                    true
                }else{
                    let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
                    helper::write_hit_small(s, hit, is_hit, hit_dice.unwrap_or(0));
                    write!(s, " ").unwrap();
                    is_hit
                };
                if is_hit {
                    board.get_unit_mut(id).untie(&bound);
                    writeln!(s, "{}", "成功".to_string().color(Color::Green)).unwrap();
                } else {
                    let new_tight = 100 - hit;
                    if tight != new_tight {
                        writeln!(s, "挣脱至 -> {}", new_tight.to_string().color(Color::Yellow)).unwrap();
                    }else{
                        writeln!(s, "{}", "失败".to_string().color(Color::Red)).unwrap();
                    }
                    board.get_unit_mut(id).tightness_change_to(&bound, new_tight);
                }
            }
        }
    }
}