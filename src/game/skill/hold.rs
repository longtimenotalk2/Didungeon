use crate::game::{unit::Unit, skill::{txt_hit, txt_announce, Skill}};

use super::{BASIC_HIT, HIT_RATE, to_hit, Skillize};

pub struct Hold {
    basic_attach_hit : i32,
    attach_hit_rate : i32,
    basic_fall_hit : i32,
    fall_hit_rate : i32,
    basic_hold_hit : i32,
    hold_hit_rate : i32,
}

impl Hold {
    pub fn new() -> Self {
        Self {
            basic_attach_hit: BASIC_HIT,
            attach_hit_rate: HIT_RATE,
            basic_fall_hit: BASIC_HIT,
            fall_hit_rate: HIT_RATE,
            basic_hold_hit: BASIC_HIT,
            hold_hit_rate: HIT_RATE,
        }
    }

    fn can(&self, a : &Unit, ib : u8) -> bool {
        if let Some(ibb) = a.catch {
            if ibb == ib {
                return false;
            }
        }
        true
    }

    fn hit_attact(&self, a : &Unit, b : &Unit) -> i32 {
        let acc = a.acc_melee_hand();
        let evd = b.evd();
        if evd == 0 {return 100};
        let hit = to_hit(self.basic_attach_hit + (acc - evd) * self.attach_hit_rate);
        hit
    }

    fn hit_fall(&self, a : &Unit, b : &Unit) -> i32 {
        let acc = a.push();
        let evd = b.anti_push();
        if evd == 0 {return 100};
        let hit = to_hit(self.basic_fall_hit + (acc - evd) * self.fall_hit_rate);
        hit
    }

    fn hit_hold(&self, a : &Unit, b : &Unit) -> i32 {
        let acc = a.hold();
        let evd = b.anti_hold();
        if evd == 0 {return 100};
        let hit = to_hit(self.basic_hold_hit + (acc - evd) * self.hold_hit_rate);
        hit
    }
}

impl Skillize for Hold {    
    fn target(&self, board : &crate::game::board::Board, ia : u8) -> Vec<u8> {
        let a = board.index(ia);
        let mv = a.mv();
        let mut ibs = vec!();
        for ib in board.find_melee_target(ia, mv) {
            if self.can(a, ib) {
                ibs.push(ib);
            }
        }
        ibs
    }

    fn evaluate(&self, board : &crate::game::board::Board, ia : u8, ib : u8) -> (i32, Option<String>) {
        let a = board.index(ia);
        let b = board.index(ib);
        let mut point = self.hit_attact(a, b);
        if !b.fall {
            point = point * self.hit_fall(a, b) / 100;
        }
        point = point * self.hit_hold(a, b) / 100;
        (point, None)
    }

    fn exe(&self, board : &mut crate::game::board::Board, ia : u8, ib : u8, dice : &mut crate::wyrand::Dice) -> String {
        let mut txt = String::new();

        txt += &txt_announce(&Skill::Hold, ib);

        board.rush_to(ia, ib);
        
        let a = board.index(ia);
        let b = board.index(ib); 
        let hit1 = self.hit_attact(a, b);
        let hit_dice = dice.d(100);
        let is_hit1 = hit1 >= hit_dice;
        txt += &format!("{}", txt_hit("attach", hit1, hit_dice, is_hit1, "success"));
        if is_hit1 {
            if !b.fall {
                let hit2 = self.hit_fall(a, b);
                let hit_dice = dice.d(100);
                let is_hit2 = hit2 >= hit_dice;
                if is_hit2 {
                    let mut b = board.index_mut(ib);
                    b.fall = true;
                }
                txt += &format!("{}", txt_hit("push opponent", hit2, hit_dice, is_hit2, "success"));
            }
            let a = board.index(ia);
            let b = board.index(ib);
            if b.fall {
                let hit3 = self.hit_hold(a, b);
                let hit_dice = dice.d(100);
                let is_hit3 = hit3 >= hit_dice;
                if is_hit3 {
                    board.hold(ia, ib)
                }
                txt += &format!("{}", txt_hit("hold opponent", hit3, hit_dice, is_hit3, "success"));
            }
        } 
        txt
    }
}