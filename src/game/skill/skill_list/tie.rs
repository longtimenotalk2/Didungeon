use colorful::{Color, Colorful};
use num_rational::Ratio;
use std::fmt::Write;

use crate::game::{unit::{Unit, bound::BoundPart, Id}, board::Board, skill::helper};

fn deal_with_hit(s : &mut String, board : &mut Board, bound_point : i32, cost_or_hit : Result<i32, i32>) -> (i32, i32, bool) {
    let (hit, cost, is_success) = match cost_or_hit {
        Ok(cost) => (100, cost, true),
        Err(hit) => {
            if hit == 100 {
                (hit, bound_point, true)
            }else{
                let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
                helper::write_hit_small(s, hit, is_hit, hit_dice.unwrap_or(0));
                write!(s, " ").unwrap();
                (hit, bound_point, is_hit)
            }
        },
    };
    (hit, cost, is_success)
}

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

    pub fn tie_choose(&self, actor : &Unit, target : &Unit) -> Vec<(BoundPart, TieWay)> {
        let mut list = vec!();
        for bound in BoundPart::all() {
            let tight = target.get_tightness(&bound);
            if 0 < tight && tight < 100 {
                list.push((bound, TieWay::Tight));
            }
        }

        for b in target.can_tie_list() {
            let effi = self.get_effi(&b, actor, target);
            if effi > 0 {
                list.push((b, TieWay::Tie));
            }
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

    pub fn exe_tight(&self, s : &mut String, bound : BoundPart, bound_point : i32, board : &mut Board, it : Id) -> i32 {
        let target = board.get_unit(it);

        // 扎紧 [...](67%) 手腕 成功 (消耗点数 : 33)
        // 扎紧 [...](10%) 手臂 (30%成功率 -> 🎲 : 71) 扎紧至 -> 30% (消耗点数 : 67)
        let name = "扎紧".color(Color::Yellow);
        let bound_idy = target.bound_identity(Some((&bound, true)), false);
        let tight_idy = target.identity_tightness(&bound);
        let bound_name_idy = bound.name();
        write!(s, "{name} {bound_idy}{tight_idy} {bound_name_idy} ").unwrap();

        let (hit, cost, is_success) = deal_with_hit(s, board, bound_point, self.tight_get_cost_or_rate(bound_point, &bound, target));

        match is_success {
            true => {
                board.get_unit_mut(it).tie(&bound);
                write!(s, "{} ", "成功".to_string().color(Color::Green)).unwrap();
            },
            false => {
                let new_tight = hit;
                board.get_unit_mut(it).tightness_change_to(&bound, new_tight);
                write!(s, "绳索强度 -> {} ", new_tight.to_string().color(Color::Yellow)).unwrap();
            },
        }
        write!(s, "消耗点数 : {}\n", cost.to_string().color(Color::Yellow)).unwrap();

        // 目标受精
        if board.get_unit_mut(it).shock() {
            writeln!(s, "{} {}!", board.get_unit(it).identity(), "受惊".to_string().color(Color::Yellow)).unwrap();
        }

        bound_point - cost
    }

    pub fn exe_untie(&self, s : &mut String, bound : BoundPart, bound_point : i32, board : &mut Board, it : Id) -> i32 {

        let target = board.get_unit(it);

        // 解绑 [...](67%) 手腕 成功 (消耗点数 : 33)
        // 解绑 [...](80%) 手臂 (70%成功率 -> 🎲 : 71) 解绑至 -> 30% (消耗点数 : 67)
        let name = "解绑".color(Color::Red);
        let bound_idy = target.bound_identity(Some((&bound, false)), false);
        let tight_idy = target.identity_tightness(&bound);
        let bound_name_idy = bound.name();
        write!(s, "{name} {bound_idy}{tight_idy} {bound_name_idy} ").unwrap();

        let (hit, cost, is_success) = deal_with_hit(s, board, bound_point, self.untie_get_cost_or_rate(bound_point, &bound, target));

        match is_success {
            true => {
                board.get_unit_mut(it).untie(&bound);
                write!(s, "{} ", "成功".to_string().color(Color::Green)).unwrap();
            },
            false => {
                let new_tight = 100 - hit;
                board.get_unit_mut(it).tightness_change_to(&bound, new_tight);
                write!(s, "解绑至 -> {} ", new_tight.to_string().color(Color::Yellow)).unwrap();
            },
        }
        write!(s, "消耗点数 : {}\n", cost.to_string().color(Color::Yellow)).unwrap();

        // 目标受精
        if board.get_unit_mut(it).shock() {
            writeln!(s, "{} {}!", board.get_unit(it).identity(), "受惊".to_string().color(Color::Yellow)).unwrap();
        }

        bound_point - cost
    }

    pub fn exe_tie(&self, s: &mut String, bound : BoundPart, bound_point : i32, board : &mut Board, id : Id, it : Id) -> i32 {
        
        let actor = board.get_unit(id);
        let target = board.get_unit(it);

        // 捆绑 [...] 手腕 成功 (消耗点数 : 33)
        // 捆绑 [...] 手臂 (30%成功率 -> 🎲 : 71) 失败 (消耗点数 : 67)
        let name = "捆绑".color(Color::Green);
        let bound_idy = target.bound_identity(Some((&bound, true)), false);
        let bound_name_idy = bound.name();
        write!(s, "{name} {bound_idy} {bound_name_idy} ").unwrap();

        let (_hit, cost, is_success) = deal_with_hit(s, board, bound_point, self.tie_get_cost_or_rate(bound_point, &bound, actor, target));

        match is_success {
            true => {
                board.get_unit_mut(it).tie(&bound);
                write!(s, "{} ", "成功".to_string().color(Color::Green)).unwrap();
            },
            false => {
                write!(s, "{} ", "失败".to_string().color(Color::Red)).unwrap();
            },
        }
        write!(s, "消耗点数 : {}\n", cost.to_string().color(Color::Yellow)).unwrap();

        // 目标受精
        if board.get_unit_mut(it).shock() {
            writeln!(s, "{} {}!", board.get_unit(it).identity(), "受惊".to_string().color(Color::Yellow)).unwrap();
        }

        bound_point - cost
    }

    pub fn end(&self, s: &mut String, board : &mut Board, id : Id, it : Id) {
        board.get_unit_mut(id).cancel_catch_with(it);
        board.get_unit_mut(it).cancel_catched_with(id);
        // 检查苏醒
        board.check_awake(s);
    }
}