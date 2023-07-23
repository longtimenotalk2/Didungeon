use colorful::{Color, Colorful};

use crate::game::{unit::{Unit, Id, Dir, bound::BoundPart}, skill::{Skillize, Skill, helper}, board::Board};

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
pub struct Untie {
    basic_dex_rate : i32
}

impl Untie {
    pub fn new() -> Self {
        Self {
            basic_dex_rate: 20,
        }
    }

    pub fn bound_point(&self, actor : &Unit) -> i32 {
        self.basic_dex_rate * actor.hand_dex()
    }

    pub fn can(&self, actor : &Unit, target : &Unit) -> bool {
        actor.hand_dex() > 0 && !actor.is_fall() && target.has_bound()
    }

    fn range(&self, actor : &Unit) -> i32 {
        actor.move_range() + 1
    }

    pub fn untie_choice(&self, target : &Unit) -> Vec<BoundPart> {
        target.can_untie_list()
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

    pub fn exe_untie(&self, s : &mut String, bound : BoundPart, bound_point : i32, board : &mut Board, it : Id) -> i32 {

        let target = board.get_unit(it);

        // 解绑 [...](67%) 手腕 成功 (消耗点数 : 33)
        // 解绑 [...](80%) 手臂 (70%成功率 -> 🎲 : 71) 解绑至 -> 30% (消耗点数 : 67)
        let bound_idy: String = target.bound_identity(Some((&bound, false)), false);
        let tight_idy = target.identity_tightness(&bound);
        let bound_name_idy = bound.name();
        write!(s, "解绑 {bound_idy}{tight_idy} {bound_name_idy} ").unwrap();

        let (hit, cost, is_success) = deal_with_hit(s, board, bound_point, self.unbound_get_cost_or_rate(bound_point, &bound, target));

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
        bound_point - cost
    }
}

impl Skillize for Untie {
    fn get_targets(&self, board : &Board, id : Id) -> Vec<(Id, Dir)> {
        let mut list = vec![];
        for (it, dir) in board.find_friend_with_range(id, self.range(board.get_unit(id))) {
            let actor = board.get_unit(id);
            let target = board.get_unit(it);
            if self.can(actor, target) {
                list.push((it, dir));
            }
        }
        list
    }

    fn exe(&self, s : &mut String, board : &mut Board, id : Id, it : Id, dir : &Dir) {
        
        let target = board.get_unit(it);

        // 宣言
        *s += &helper::write_announce(target, &dir, &Skill::Untie);
        *s += "\n";

        // 冲刺
        board.dash_to(id, it, dir);

        let actor = board.get_unit(id);
        let bound_point = self.bound_point(actor);

        // 结算
        board.set_to_untie(id, it, bound_point)
    }

    fn choice_show(&self, board : &Board, _id : Id, it : Id, dir : &Dir) -> String {
        let mut st = String::new();

        let target = board.get_unit(it);
        st += &helper::write_announce( target, dir, &Skill::Untie);

        st
    }

    fn analyse(&self, board : &Board, id : Id, it : Id, dir : &Dir) -> Board {
        let mut board = board.clone();
        // 冲刺
        board.dash_to(id, it, dir);
        board
    }
}