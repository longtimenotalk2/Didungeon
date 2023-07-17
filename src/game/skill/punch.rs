use crate::game::{board::Board, unit::{Id, Dir, Unit}};

use super::{Skillize, Skill};

use super::helper as helper;

pub struct Punch {
    basic_hit : i32,
    hit_rate : i32,
    basic_dmg : i32,
}

impl Punch {
    pub fn new() -> Self {
        Self {
            basic_hit: 50,
            hit_rate: 5,
            basic_dmg: 10,
        }
    }

    fn can(&self, actor : &Unit) -> bool {
        actor.acc_melee_hand() > 0
    }

    fn hit(&self, actor : &Unit, target : &Unit) -> i32 {
        let acc = actor.acc_melee_hand();
        let evd = target.evd();
        if evd == 0 {
            return 100;
        }
        helper::to_hit(self.basic_hit + self.hit_rate * (acc - evd))
    }

    fn dmg(&self, actor : &Unit, target : &Unit) -> i32 {
        let atk = actor.atk_melee_hand();
        let def = target.def_gym();
        helper::to_dmg(self.basic_dmg + atk - def, 1)
    }

    fn stun_rate(&self, actor : &Unit, target : &Unit) -> i32 {
        self.dmg(actor, target)
    }
}

impl Skillize for Punch {
    fn get_targets(&self, board : &Board, id : Id) -> Vec<(Id, Dir)> {
         if self.can(board.get_unit(id)) {
            board.find_adjs(id)
         }else{
            vec!()
         }
    }

    fn exe(&self, board : &mut Board, id : Id, it : Id, dir : &Dir) {
        let actor = board.get_unit(id);
        let target = board.get_unit(it);

        // 宣言
        helper::show_announce(actor, target, &dir, &Skill::Punch);

        // 命中判定
        let hit = self.hit(actor, target);
        let (is_hit, hit_dice) = helper::hit_check(hit, board.get_dice());
        helper::show_hit(hit, is_hit, hit_dice, "命中率", "命中", "落空");

        // 命中结算
        if is_hit {
            // 受伤
            let actor = board.get_unit(id);
            let target = board.get_unit(it);
            let dmg = self.dmg(actor, target);
            let inj_old = target.get_inj();
            let inj_new = board.get_unit_mut(it).take_dmg(dmg);
            helper::show_dmg(dmg, inj_old, inj_new);

            // 击晕
            let actor = board.get_unit(id);
            let target = board.get_unit(it);
            let stun_rate = self.stun_rate(actor, target);
            let (is_stun, stun_dice) = helper::hit_check(stun_rate, board.get_dice());
            if is_stun {
                board.get_unit_mut(it).take_stun();
            }
            helper::show_hit(stun_rate, is_stun, stun_dice, "击晕率", "击晕", "落空");
        }
    }
}
