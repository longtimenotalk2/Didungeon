use crate::game::{unit::{Bound, Unit}, board::Board};

use super::{Skillize, to_hit, txt_hit, HIT_RATE, BASIC_HIT};

pub struct Tie {
    basic_hit : i32,
    hit_rate : i32,
}

impl Tie {
    pub fn new() -> Self {
        Self {
            basic_hit: BASIC_HIT,
            hit_rate: HIT_RATE,
        }
    }

    pub fn can(&self, a : &Unit, b : &Unit) -> bool {
        if a.ally == b.ally {return false;}
        if a.bound_wrist {return false};
        if b.stun {return true};
        if !b.hold {return false};
        b.next_can_tie_choices().len() > 0
    }

    pub fn hit(&self, a : &Unit, b : &Unit, bd : &Bound) -> i32 {
        let acc = a.tie_power();
        let evd = if bd.is_upper() {b.anti_tie_upper()} else {b.anti_tie_lower()};
        let hit = to_hit(self.basic_hit + (acc - evd) * self.hit_rate);
        hit 
    }

    pub fn times_and_remain_hit(&self, agi : i32) -> (i32, i32) {
        let times = agi / 5;
        let hit = (agi - times * 5) * 20;
        (times, hit)
    }

    pub fn choice(&self, board : &crate::game::board::Board, ia : u8, ib : u8) -> Option<(Bound, bool, i32)> {
        let a = board.index(ia);
        let b = board.index(ib);
        let choices = b.next_can_tie_choices();
        let mut choice : Option<(Bound, bool, i32)> = None;
        for ch in choices {
            let (bd, is_tie) = ch;
            let hit = self.hit(a, b, &bd);
            match choice {
                Some((_, _, hit_)) => {
                    if hit > hit_ {
                        choice = Some((bd, is_tie, hit));
                    }
                },
                None => choice = Some((bd, is_tie, hit)),
            }
        }
        choice
    }
}

impl Skillize for Tie {

    fn target(&self, board : &crate::game::board::Board, ia : u8) -> Vec<u8> {
        let a = board.index(ia);
        let mut ibs = vec!();
        for ib in board.find_melee_target(ia, 0) {
            let b = board.index(ib);
            if self.can(a, b) {
                ibs.push(ib);
            }
        }
        ibs
    }

    fn evaluate(&self, board : &crate::game::board::Board, ia : u8, ib : u8) -> (i32, Option<String>) {
        if let Some((_bd, _is_tie, hit)) = self.choice(board, ia, ib) {
            let a = board.index(ia);
            let times = a.hand_agi() as f32 / 5. ;
            let txt = format!("{hit}% x {:2}", times);
            
            (hit, Some(txt))
        }else{
            (0, None)
        }
    }

    fn exe(&self, board : &mut crate::game::board::Board, ia : u8, ib : u8, dice : &mut crate::wyrand::Dice) -> String {
        let mut txt = String::new();

        // free hold when stun
        let b = board.index_mut(ib);
        if b.stun && !b.hold {
            board.hold(ia, ib);
            txt += "<auto hold>\n";
        }
        
        let a = board.index(ia);
        let agi = a.hand_agi();
        let (times, hit) = self.times_and_remain_hit(agi);
        let hit_dice = dice.d(100);
        let is_hit = hit >= hit_dice;
        let new_times = if is_hit {times + 1}else{times};
        txt += &format!("<tie x {new_times} -> {ib}> ({times} + {hit}% -> d100 : {hit_dice})\n");
        
        for _ in 0..new_times {
            if let Some((bd, to_tie, hit)) = self.choice(board, ia, ib) {
                let hit_dice = dice.d(100);
                let is_hit = hit > hit_dice;
                if is_hit {
                    let b = board.index_mut(ib);
                    match bd {
                        Bound::Neck => b.bound_neck = to_tie,
                        Bound::Arm => b.bound_arm = to_tie,
                        Bound::Hang => b.bound_hang = to_tie,
                        Bound::Wrist => b.bound_wrist = to_tie,
                        Bound::Joint => b.bound_joint = to_tie,
                        Bound::Thigh => b.bound_thigh = to_tie,
                        Bound::Calve => b.bound_calve = to_tie,
                        Bound::Ankle => b.bound_ankle = to_tie,
                        Bound::Long => b.bound_long = to_tie,
                    }
                }
                let to_tie_txt = if to_tie {"tie"} else {"untie"};
                let target = format!("{to_tie_txt} {}", bd.txt());
                txt += &format!("{}", txt_hit(&target, hit, hit_dice, is_hit, &board.index(ib).txt_bound()));
                if is_hit {
                    let b = board.index(ib);
                    txt += &format!("{}{}\n", Board::title_front(), b.txt_attr());
                }

            }
        }

        txt
    }
}