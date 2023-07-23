use std::vec;

use colorful::{Color, Colorful};

use crate::game::{skill::{helper, Skillize, Skill}, unit::{Unit, Dir, Id, bound::BoundPart}, board::Board};

use std::fmt::Write;

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

pub struct Unbound {
    basic_dex_rate : i32
}

impl Unbound {
    pub fn new() -> Self {
        Self {
            basic_dex_rate: 20,
        }
    }

    pub fn bound_point(&self, actor : &Unit) -> i32 {
        self.basic_dex_rate * actor.hand_dex()
    }

    pub fn unbound_choice(&self, actor : &Unit) -> Vec<BoundPart> {
        actor.can_untie_list()
    }

    pub fn unbound_get_cost_or_rate(&self, bound_point : i32, bound : &BoundPart, actor : &Unit) -> Result<i32, i32> {
        let ramain_tightness = actor.get_tightness(bound);
        let cost = ramain_tightness;
        if cost < bound_point {
            return Ok(cost)
        }else{
            return Err(100 + bound_point - ramain_tightness)
        }
    }

    pub fn exe_unbound(&self, s : &mut String, bound : BoundPart, bound_point : i32, board : &mut Board, id : Id) -> i32 {

        let actor = board.get_unit(id);

        // 解绑 [...](67%) 手腕 成功 (消耗点数 : 33)
        // 解绑 [...](80%) 手臂 (70%成功率 -> 🎲 : 71) 解绑至 -> 30% (消耗点数 : 67)
        let bound_idy: String = actor.bound_identity(Some((&bound, false)), false);
        let tight_idy = actor.identity_tightness(&bound);
        let bound_name_idy = bound.name();
        write!(s, "解绑 {bound_idy}{tight_idy} {bound_name_idy} ").unwrap();

        let (hit, cost, is_success) = deal_with_hit(s, board, bound_point, self.unbound_get_cost_or_rate(bound_point, &bound, actor));

        match is_success {
            true => {
                board.get_unit_mut(id).untie(&bound);
                write!(s, "{} ", "成功".to_string().color(Color::Green)).unwrap();
            },
            false => {
                let new_tight = 100 - hit;
                board.get_unit_mut(id).tightness_change_to(&bound, new_tight);
                write!(s, "解绑至 -> {} ", new_tight.to_string().color(Color::Yellow)).unwrap();
            },
        }
        write!(s, "消耗点数 : {}\n", cost.to_string().color(Color::Yellow)).unwrap();
        bound_point - cost
    }
}

impl Skillize for Unbound {
    fn get_targets(&self, board : &Board, id : Id) -> Vec<(Id, Dir)> {
        let actor = board.get_unit(id);
        if actor.has_bound() && self.bound_point(actor) > 0 {
            vec![(id, actor.get_dir())]
        }else{
            vec![]
        }
    }

    fn exe(&self, _s : &mut String, board : &mut Board, id : Id, _it : Id, _dir : &Dir) {
        let actor = board.get_unit(id);
        let bound_point = self.bound_point(actor);
        board.set_to_unbound(id, bound_point);
    }
    
    fn choice_show(&self, board : &Board, id : Id, _it : Id, dir : &Dir) -> String {
        let mut st = String::new();

        let target = board.get_unit(id);
        st += &helper::write_announce( target, dir, &Skill::Unbound);
        st
    }

    fn analyse(&self, board : &Board, _id : Id, _it : Id, _dir : &Dir) -> Board {
        board.clone()
    }
}