use crate::{game::{board::Board, unit::Unit}, wyrand::Dice};

use super::{BASIC_HIT, HIT_RATE, to_hit, to_dmg, txt_hit, Skillize, txt_announce, Skill};

pub struct Punch {
    basic_hit : i32,
    hit_rate : i32,
    basic_dmg : i32,
}

impl Punch {
    pub fn new() -> Self {
        Self {
            basic_hit: BASIC_HIT,
            hit_rate: HIT_RATE,
            basic_dmg: 10,
        }
    }

    fn can(&self, a : &Unit, b : &Unit) -> bool {
        if a.ally == b.ally {return false;}
        if a.fall {return false};
        if a.bound_wrist {return false};
        if b.is_defeated() {return false};
        true
    }

    fn hit(&self, acc : i32, evd : i32) -> i32 {
        to_hit(self.basic_hit + self.hit_rate * (acc - evd))
    }

    fn dmg(&self, atk : i32, def : i32) -> i32 {
        to_dmg(self.basic_dmg + atk - def, 1)
    }

    fn stun_rate(&self, atk : i32, def : i32) -> i32 {
        to_dmg(self.basic_dmg + atk - def, 1)
    }
}

impl Skillize for Punch {
    fn target(&self, board : &crate::game::board::Board, ia : u8) -> Vec<u8> {
        let a = board.index(ia);
        let mv = a.mv();
        let mut ibs = vec!();
        for ib in board.find_melee_target(ia, mv) {
            let b = board.index(ib);
            if self.can(a, b) {
                ibs.push(ib);
            }
        }
        ibs
    }

    fn evaluate(&self, board : &Board, ia : u8, ib : u8) -> (i32, Option<String>) {
        let a = board.index(ia);
        let b = board.index(ib);
        let acc = a.acc_melee_hand();
        let evd = b.evd();
        let hit = self.hit(acc, evd);
        let atk = a.hand_str();
        let def = b.hand_str();
        let dmg = self.dmg(atk, def);

        let point = (hit * dmg / (dmg + 1)).min(80);
        let txt = format!("{hit}% x {dmg}");
        (point, Some(txt))
    }

    fn exe(&self, board : &mut Board, ia : u8, ib : u8, dice : &mut Dice) -> String {
        let mut txt = String::new();
        let a = board.index(ia);
        let b = board.index(ib);

        txt += &txt_announce(&Skill::Punch, ib);

        let acc = a.acc_melee_hand();
        let evd = b.evd();
        let hit = self.hit(acc, evd);
        let atk = a.hand_str();
        let def = b.hand_str();
        let dmg = self.dmg(atk, def);
        let stun_rate = self.stun_rate(atk, def);

        let hit_dice = dice.d(100);
        let is_hit = hit >= hit_dice;
        if is_hit {
            board.index_mut(ib).inj += dmg;
        }
        let inj = board.index(ib).inj;
        txt += &format!("{}", txt_hit("punch", hit, hit_dice, is_hit, &format!("{dmg} dmg -> {inj}")));

        // stun
        if is_hit {
            let hit_dice = dice.d(100);
            let is_hit = stun_rate >= hit_dice;
            if is_hit {
                let b = board.index_mut(ib);
                b.be_stun();
                b.action = false;
            }
            txt += &format!("{}", txt_hit("stun check", dmg, hit_dice, is_hit, &format!("stun!")));
        }

        txt
    }
}