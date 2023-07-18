use colorful::{Color, Colorful};
use num_rational::Ratio;

use crate::game::{unit::{Unit, bound::BoundPart, Id}, board::Board, skill::helper};

pub struct Tie {
    basic_effi : i32,
    effi_rate : i32,
    basic_dex_rate : i32
}

pub enum TieWay {
    Tight,
    Tie,
    Untie,
}

impl Tie {
    pub fn new() -> Self {
        Self {
            basic_effi: 50,
            effi_rate: 5,
            basic_dex_rate: 20,
        }
    }

    pub fn bound_point(&self, actor : &Unit) -> i32 {
        actor.hand_dex() * self.basic_dex_rate
    }

    pub fn tie_choose(&self, target : &Unit) -> Vec<(BoundPart, TieWay)> {
        let mut list = vec!();
        for bound in BoundPart::all() {
            let tight = target.get_tightness(&bound);
            if 0 < tight && tight < 100 {
                list.push((bound, TieWay::Tight));
            }
        }

        for b in target.can_tie_list() {
            list.push((b, TieWay::Tie));
        }
        for b in target.can_untie_list() {
            list.push((b, TieWay::Untie));
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

    pub fn tie_get_cost_or_rate(&self, bound_point : i32, bound : &BoundPart, actor : &Unit, target : &Unit) -> Result<i32, i32> {
        let effi = self.get_effi(&bound, actor, target);
        if effi == 0 {
            return Err(0);
        }
        let effi = Ratio::new(effi, 100);
        let cost = Ratio::from_integer(100) / effi;
        let cost = cost.ceil().to_integer();
        if cost < bound_point {
            return Ok(cost);
        }else{
            let hit = Ratio::new(bound_point * 100, cost).ceil().to_integer();
            return Err(hit);
        }
    }

    pub fn untie_get_cost_or_rate(&self, bound_point : i32, bound : &BoundPart, target : &Unit) -> Result<i32, i32> {
        let ramain_tightness = target.get_tightness(bound);
        let cost = ramain_tightness;
        if cost < bound_point {
            return Ok(cost)
        }else{
            return Err(100 + bound_point - ramain_tightness)
        }
    }

    pub fn tight_get_cost_or_rate(&self, bound_point : i32, bound : &BoundPart, target : &Unit) -> Result<i32, i32> {
        let ramain_tightness = target.get_tightness(bound);
        let cost = 100 - ramain_tightness;
        if cost < bound_point {
            return Ok(cost)
        }else{
            return Err(bound_point + ramain_tightness)
        }
    }

    pub fn exe_pass(&self, board : &mut Board, id : Id, it : Id) {
        board.get_unit_mut(id).cancel_catch_with(it);
        board.get_unit_mut(it).cancel_catched_with(id);
        board.get_unit_mut(it).awake();
    }

    pub fn exe_tight(&self, bound : BoundPart, bound_point : i32, board : &mut Board, id : Id, it : Id) -> i32 {
        let target = board.get_unit(it);

        match self.tight_get_cost_or_rate(bound_point, &bound, target) {
            Ok(cost) => {
                println!("尝试加固 {} {} {}", target.identity(), target.bound_identity_change(&bound, true), bound.name());
                println!("加固成功率 : 100% -> {}", "成功".to_string().color(Color::Green));
                board.get_unit_mut(it).tightness_change_to(&bound, 100);
                println!();
                board.show(Some(id));
                println!();
                bound_point - cost
            },
            Err(hit) => {
                // 命中判定
                let is_hit = if hit == 100{
                    println!("加固成功率 : 100% -> {}", "成功".to_string().color(Color::Green));
                    true
                }else if hit == 0{
                    println!("加固成功率 : 0% -> {}", "失败".to_string().color(Color::Red));
                    false
                }else{
                    let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
                    helper::show_hit(hit, is_hit, hit_dice, "加固成功率", "成功", "失败");
                    is_hit
                };

                if is_hit {
                    board.get_unit_mut(it).untie(&bound);
                }else{
                    let target = board.get_unit(it);
                    let tight = target.get_tightness(&bound);
                    let new_tight = hit;
                    println!("绳索强度 : {} -> {}", tight, new_tight.to_string().color(Color::Green));
                    board.get_unit_mut(it).tightness_change_to(&bound, hit);
                    
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

    pub fn exe_untie(&self, bound : BoundPart, bound_point : i32, board : &mut Board, id : Id, it : Id) -> i32 {

        let target = board.get_unit(it);

        match self.untie_get_cost_or_rate(bound_point, &bound, target) {
            Ok(cost) => {
                println!("尝试解绑 {} {} {}", target.identity(), target.bound_identity_change(&bound, false), bound.name());
                println!("解绑成功率 : 100% -> {}", "成功".to_string().color(Color::Green));
                board.get_unit_mut(it).untie(&bound);
                println!();
                board.show(Some(id));
                println!();
                bound_point - cost
            },
            Err(hit) => {
                // 命中判定
                let is_hit = if hit == 100{
                    println!("解绑成功率 : 100% -> {}", "成功".to_string().color(Color::Green));
                    true
                }else if hit == 0{
                    println!("解绑成功率 : 0% -> {}", "失败".to_string().color(Color::Red));
                    false
                }else{
                    let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
                    helper::show_hit(hit, is_hit, hit_dice, "解绑成功率", "成功", "失败");
                    is_hit
                };

                if is_hit {
                    board.get_unit_mut(it).untie(&bound);
                }else{
                    let target = board.get_unit(it);
                    let tight = target.get_tightness(&bound);
                    let new_tight = 100 - hit;
                    println!("绳索强度 : {} -> {}", tight, new_tight.to_string().color(Color::Green));
                    board.get_unit_mut(it).tightness_change_to(&bound, new_tight);
                    
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

    pub fn exe_tie(&self, bound : BoundPart, bound_point : i32, board : &mut Board, id : Id, it : Id) -> i32 {
        let actor = board.get_unit(id);
        let target = board.get_unit(it);

        match self.tie_get_cost_or_rate(bound_point, &bound, actor, target) {
            Ok(cost) => {
                println!("尝试捆绑 {} {} {}", target.identity(), target.bound_identity_change(&bound, true), bound.name());
                println!("捆绑成功率 : 100% -> {}", "成功".to_string().color(Color::Green));
                board.get_unit_mut(it).tie(&bound);
                println!();
                board.show(Some(id));
                println!();
                bound_point - cost
            },
            Err(hit) => {
                println!("尝试捆绑 {} {} {}", target.identity(), target.bound_identity_change(&bound, true), bound.name());
                // 命中判定
                let is_hit = if hit == 100 {
                    println!("捆绑成功率 : 100% -> {}", "成功".to_string().color(Color::Green));
                    true
                }else if hit == 0{
                    println!("捆绑成功率 : 0% -> {}", "失败".to_string().color(Color::Red));
                    false
                }else{
                    let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
                    helper::show_hit(hit, is_hit, hit_dice, "捆绑成功率", "成功", "失败");
                    is_hit
                };

                if is_hit {
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