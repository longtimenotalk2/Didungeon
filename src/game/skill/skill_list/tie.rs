use colorful::{Color, Colorful};
use num_rational::Ratio;

use crate::game::{unit::{Unit, bound::BoundPart, Id}, board::Board, skill::helper};

pub struct Tie {
    basic_effi : i32,
    effi_rate : i32,
    basic_dex_rate : i32
}

impl Tie {
    pub fn new() -> Self {
        Self {
            basic_effi: 50,
            effi_rate: 5,
            basic_dex_rate: 20,
        }
    }

    pub fn bound_num(&self, actor : &Unit) -> i32 {
        actor.hand_dex() * self.basic_dex_rate
    }

    pub fn tie_choose(&self, target : &Unit) -> Vec<(BoundPart, bool)> {
        let mut list = vec!();
        for b in target.can_tie_list() {
            list.push((b, true));
        }
        for b in target.can_untie_list() {
            list.push((b, false));
        }
        list
    }

    fn get_effi(&self, bound : &BoundPart, actor : &Unit, target : &Unit) -> i32 {
        let is_upper = match bound {
            BoundPart::Neck => true,
            BoundPart::Arm => true,
            BoundPart::Hang => true,
            BoundPart::Wrist => true,
            BoundPart::Joint => false,
            BoundPart::Thigh => false,
            BoundPart::Calve => false,
            BoundPart::Ankle => false,
            BoundPart::Long => false,
        };
        let evd = match is_upper {
            true => target.anti_tie_upper(),
            false => target.anti_tie_lower(),
        };
        let acc = actor.hand_str();
        helper::to_hit(self.basic_effi + self.effi_rate * (acc - evd))
    }

    pub fn tie_get_cost_or_rate(&self, bound_num : i32, bound : &BoundPart, actor : &Unit, target : &Unit) -> Result<i32, i32> {
        let effi = self.get_effi(&bound, actor, target);
        if effi == 0 {
            return Err(0);
        }
        let effi = Ratio::new(effi, 100);
        let cost = Ratio::from_integer(100) / effi;
        let cost = cost.ceil().to_integer();
        if cost < bound_num {
            return Ok(cost);
        }else{
            let hit = Ratio::new(bound_num * 100, cost).ceil().to_integer();
            return Err(hit);
        }
    }

    pub fn untie_get_cost_or_rate(&self, bound_num : i32, bound : &BoundPart, target : &Unit) -> Result<i32, i32> {
        let ramain_tightness = target.get_tightness(bound);
        let cost = ramain_tightness;
        if cost < bound_num {
            return Ok(cost)
        }else{
            return Err(100 + bound_num - ramain_tightness)
        }
    }

    pub fn exe_untie(&self, bound : BoundPart, bound_num : i32, board : &mut Board, id : Id, it : Id) -> i32 {
        let target = board.get_unit(it);

        match self.untie_get_cost_or_rate(bound_num, &bound, target) {
            Ok(cost) => {
                println!("解绑 {} {}", target.identity(), bound.name_untie(), );
                board.get_unit_mut(it).untie(&bound);
                println!();
                board.show(Some(id));
                println!();
                bound_num - cost
            },
            Err(hit) => {
                // 命中判定
                let is_hit = if hit == 100{
                    true
                }else if hit == 0{
                    println!("解绑失败！");
                    false
                }else{
                    let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
                    helper::show_hit(hit, is_hit, hit_dice, "解绑成功率", "成功", "失败");
                    is_hit
                };

                if is_hit {
                    let target = board.get_unit(it);
                    println!("解绑 {} {}", target.identity(), bound.name_untie());
                    board.get_unit_mut(it).untie(&bound);
                }else{
                    let target = board.get_unit(it);
                    let tight = target.get_tightness(&bound);
                    let new_tight = 100 - hit;
                    println!("解绑 {} {} {}, 进度推进 {} -> {}", target.identity(), bound.name_untie(), "失败".to_string().color(Color::Red), tight, new_tight.to_string().color(Color::Green));
                    board.get_unit_mut(it).loosen_to(&bound, new_tight);
                    
                }
                board.get_unit_mut(id).cancel_catch_with(it);
                board.get_unit_mut(it).cancel_catched_with(id);
                board.get_unit_mut(it).awake();
                println!();
                board.show(Some(id));
                println!();
                0
            },
        }
    }

    pub fn exe_tie(&self, bound : BoundPart, bound_num : i32, board : &mut Board, id : Id, it : Id) -> i32 {
        let actor = board.get_unit(id);
        let target = board.get_unit(it);

        match self.tie_get_cost_or_rate(bound_num, &bound, actor, target) {
            Ok(cost) => {
                println!("捆绑 {} {}", target.identity(), bound.name_tie(), );
                board.get_unit_mut(it).tie(&bound);
                println!();
                board.show(Some(id));
                println!();
                bound_num - cost
            },
            Err(hit) => {
                // 命中判定
                let is_hit = if hit == 100{
                    true
                }else if hit == 0{
                    println!("捆绑失败！");
                    false
                }else{
                    let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
                    helper::show_hit(hit, is_hit, hit_dice, "捆绑成功率", "成功", "失败");
                    is_hit
                };

                if is_hit {
                    let target = board.get_unit(it);
                    println!("对 {} {}", target.identity(), bound.name_tie());
                    board.get_unit_mut(it).tie(&bound);
                }
                board.get_unit_mut(id).cancel_catch_with(it);
                board.get_unit_mut(it).cancel_catched_with(id);
                board.get_unit_mut(it).awake();
                println!();
                board.show(Some(id));
                println!();
                0
            }
        }
    }
}