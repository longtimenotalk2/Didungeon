use crate::game::{unit::{Unit, Bound}, board::Board};

use super::{to_hit, Skillize, txt_hit};

pub struct Untie {
    basic_hit : i32,
    hit_rate : i32,
}

impl Untie {
    pub fn new() -> Self {
        Self {
            basic_hit: 0,
            hit_rate: 10,
        }
    }

    pub fn can(&self, a : &Unit, b : &Unit) -> bool {
        if a.ally != b.ally {return false;}
        if a.fall {return false};
        b.next_untie().is_some()
    }

    pub fn hit(&self, a : &Unit) -> i32 {
        let acc = a.hand_dex();
        to_hit(self.basic_hit + self.hit_rate * acc)
    }

    pub fn times_and_remain_hit(&self, agi : i32) -> (i32, i32) {
        let times = agi / 5;
        let hit = (agi - times * 5) * 20;
        (times, hit)
    }
}

impl Skillize for Untie {
    fn target(&self, board : &crate::game::board::Board, ia : u8) -> Vec<u8> {
        let a = board.index(ia);
        let mut ibs = vec!();
        for ib in board.find_adjacent(ia) {
            let b = board.index(ib);
            if self.can(a, b) {
                ibs.push(ib);
            }
        }
        ibs
    }

    fn evaluate(&self, board : &crate::game::board::Board, ia : u8, _ib : u8) -> (i32, Option<String>) {
        let a = board.index(ia);
        (self.hit(a), None)
    }

    fn exe(&self, board : &mut crate::game::board::Board, ia : u8, ib : u8, dice : &mut crate::wyrand::Dice) -> String {
        let mut txt = String::new();
        
        let a = board.index(ia);
        let agi = a.hand_agi();
        let (times, hit) = self.times_and_remain_hit(agi);
        let hit_dice = dice.d(100);
        let is_hit = hit >= hit_dice;
        let new_times = if is_hit {times + 1}else{times};
        txt += &format!("<untie x {new_times} -> {ib}> ({times} + {hit}% -> d100 : {hit_dice})\n");

        let hit = self.hit(a);

        for _ in 0..new_times {
            let b = board.index(ib);
            if let Some(bd) = b.next_untie() {
                let hit_dice = dice.d(100);
                let is_hit = hit > hit_dice;
                if is_hit {
                    let b = board.index_mut(ib);
                    match bd {
                        Bound::Neck => {
                            b.bound_neck = false;
                            b.bound_hang = false;
                            b.bound_long = false;
                        },
                        Bound::Arm => b.bound_arm = false,
                        Bound::Hang => b.bound_hang = false,
                        Bound::Wrist => {
                            b.bound_wrist = false;
                            b.bound_hang = false;
                            b.bound_joint = false;
                        },
                        Bound::Joint => b.bound_joint = false,
                        Bound::Thigh => b.bound_thigh = false,
                        Bound::Calve => b.bound_calve = false,
                        Bound::Ankle =>  {
                            b.bound_ankle = false;
                            b.bound_joint = false;
                            b.bound_long = false;
                        },
                        Bound::Long => b.bound_long = false,
                    }
                }
                let target = format!("untie {}", bd.txt());
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