use crate::game::{unit::Bound, board::Board};

use super::{Skillize, to_hit, txt_hit};

pub struct Tie {
    // weight_neck : i32,
    // weight_arm : i32,
    // weight_hang : i32,
    // weight_wrist : i32,
    // weight_joint : i32,
    // weight_thigh : i32,
    // weight_calve : i32,
    // weight_ankle : i32,
    // weight_long : i32,
}

impl Tie {
    pub fn new() -> Self {
        Self {
            // weight_neck: 100,
            // weight_arm: 100,
            // weight_hang: 100,
            // weight_wrist: 100,
            // weight_joint: 100,
            // weight_thigh: 100,
            // weight_calve: 100,
            // weight_ankle: 100,
            // weight_long: 100,
        }
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
        let acc = a.tie_power();
        for ch in choices {
            let (bd, is_tie) = ch;
            let evd = if bd.is_upper() {b.anti_tie_upper()} else {b.anti_tie_lower()};
            let hit = to_hit(50 + (acc - evd) * 5);
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
    fn can(&self, board : &crate::game::board::Board, ia : u8, ib : u8) -> bool {
        let a = board.index(ia);
        let b = board.index(ib);
        // if !b.hold {return false};
        if a.bound_wrist {return false};
        b.next_can_tie_choices().len() > 0
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
        
        let a = board.index(ia);
        let agi = a.hand_agi();
        let (times, hit) = self.times_and_remain_hit(agi);
        let hit_dice = dice.d(100);
        let is_hit = hit >= hit_dice;
        let new_times = if is_hit {times + 1}else{times};
        txt += &format!("<tie x {new_times}> (({times} + {hit}% -> d100 : {hit_dice}))");
        
        for _ in 0..times {
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