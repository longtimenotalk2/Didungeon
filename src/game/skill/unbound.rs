use crate::{game::{unit::{Unit, Bound}, board::Board}, wyrand::Dice};

use super::{Skillize, to_hit, txt_hit};

pub struct Unbound {
    basic_force_hit : i32,
    force_hit_rate : i32,
    basic_hand_hit : i32,
    hand_hit_rate : i32,
    basic_rub_hit : i32,
    rub_hit_rate : i32,
}

impl Unbound {
    pub fn new() -> Self {
        Self {
            basic_force_hit: 0,
            force_hit_rate: 5,
            basic_hand_hit: 0,
            hand_hit_rate: 5,
            basic_rub_hit: 0,
            rub_hit_rate: 1,
        }
    }

    fn can_force_upper(&self, a : &Unit) -> bool {
        if a.bound_ankle && a.bound_arm && a.bound_hang && a.bound_wrist {return false};
        a.next_force_upper().is_some()
    }

    fn can_force_lower(&self, a : &Unit) -> bool {
        if a.bound_ankle && a.bound_thigh && a.bound_calve {return false};
        a.next_force_lower().is_some()
    }

    fn can_hand(&self, a : &Unit) -> bool {
        !a.bound_wrist && !a.hold && a.next_hand().is_some()
    }

    fn can_rub(&self, a : &Unit) -> bool {
        !a.fall && a.bound_hang
    }

    fn hit_force_upper(&self, a : &Unit) -> i32 {
        to_hit(self.basic_force_hit + a.unbound_force_upper() * self.force_hit_rate)
    }

    fn hit_force_lower(&self, a : &Unit) -> i32 {
        to_hit(self.basic_force_hit + a.unbound_force_lower() * self.force_hit_rate)
    }

    fn hit_hand(&self, a : &Unit) -> i32 {
        to_hit(self.basic_hand_hit + a.hand_dex() * self.hand_hit_rate)
    }

    fn hit_rub(&self, a : &Unit) -> i32 {
        to_hit(self.basic_rub_hit + a.spd() * self.rub_hit_rate)
    }

    fn hand_times_and_remain_hit(&self, a : &Unit) -> (i32, i32) {
        if self.can_hand(a) {
            let agi = a.hand_agi();
            let times = agi / 5;
            let hit = (agi - times * 5) * 20;
            (times, hit)
        }else{
            (0, 0)
        }
    }

    fn unbound_helper(&self, board : &mut Board, ia : u8, bd : &Bound, ubd_type : &UnboundType, txt : &mut String, dice : &mut Dice) {
        let a = board.index(ia);
        let hit = to_hit(match ubd_type {
            UnboundType::ForceUpper => self.hit_force_upper(a),
            UnboundType::ForceLower => self.hit_force_lower(a),
            UnboundType::Hand => self.hit_hand(a),
            UnboundType::Cuter => self.hit_rub(a),
        });
        let hit_dice = dice.d(100);
        let is_hit = hit >= hit_dice;
        if is_hit {
            let a = board.index_mut(ia);
            match bd {
                Bound::Neck => {
                    a.bound_neck = false;
                    a.bound_hang = false;
                    a.bound_long = false;
                },
                Bound::Arm => a.bound_arm = false,
                Bound::Hang => a.bound_hang = false,
                Bound::Wrist => {
                    a.bound_wrist = false;
                    a.bound_hang = false;
                    a.bound_joint = false;
                },
                Bound::Joint => a.bound_joint = false,
                Bound::Thigh => a.bound_thigh = false,
                Bound::Calve => a.bound_calve = false,
                Bound::Ankle =>  {
                    a.bound_ankle = false;
                    a.bound_joint = false;
                    a.bound_long = false;
                },
                Bound::Long => a.bound_long = false,
            }
        }
        let a = board.index(ia);
        *txt += &txt_hit(bd.txt(), hit, hit_dice, is_hit, &a.txt_bound());
        if is_hit {
            *txt += &format!("{}{}\n", Board::title_front(), a.txt_attr())
        }
    }
}

enum UnboundType {
    ForceUpper,
    ForceLower,
    Hand,
    Cuter,
}

impl Skillize for Unbound {
    fn can(&self, board : &crate::game::board::Board, ia : u8, _ib : u8) -> bool {
        let a = board.index(ia);
        self.can_force_upper(a) || self.can_force_lower(a) || self.can_hand(a) || self.can_rub(a)
    }

    fn evaluate(&self, board : &crate::game::board::Board, ia : u8, _ib : u8) -> (i32, Option<String>) {
        let a = board.index(ia);
        let mut point = 0;
        if self.can_force_upper(a) {
            point = point.max(self.hit_force_upper(a));
        };
        if self.can_force_lower(a) {
            point = point.max(self.hit_force_lower(a));
        };
        if self.can_hand(a) {
            point = point.max(self.hit_hand(a));
        }
        if self.can_rub(a) {
            point = point.max(self.hit_rub(a));
        }

        (point, None)
    }

    fn exe(&self, board : &mut crate::game::board::Board, ia : u8, _ib : u8, dice : &mut crate::wyrand::Dice) -> String {
        let mut txt = String::new();

        let a = board.index(ia);

        if self.can_rub(a) {
            txt += &format!("<unbound hang with cuter>\n");
            self.unbound_helper(board, ia, &Bound::Hang, &UnboundType::Cuter, &mut txt, dice);
        }

        if let Some(bd) = board.index(ia).next_force_upper() {
            txt += &format!("<unbound upper with force>\n");
            self.unbound_helper(board, ia, &bd, &UnboundType::ForceUpper, &mut txt, dice);
        }

        if let Some(bd) = board.index(ia).next_force_lower() {
            txt += &format!("<unbound lower with force>\n");
            self.unbound_helper(board, ia, &bd, &UnboundType::ForceLower, &mut txt, dice);
        }

        let a = board.index(ia);
        if self.can_hand(a) {
            if let Some(_) = board.index(ia).next_hand() {
                let (times, hit) = self.hand_times_and_remain_hit(a);
                
                let hit_dice = dice.d(100);
                let is_hit = hit >= hit_dice;
                let new_times = if is_hit {times + 1}else{times};
                
                txt += &format!("<unbound with hand x {new_times}> ({times} + {hit}% -> d100 : {hit_dice})\n");
                
                for _ in 0..new_times {
                    let a = board.index(ia);
                    if let Some(bd) = a.next_hand() {
                        self.unbound_helper(board, ia, &bd, &UnboundType::Hand, &mut txt, dice);
                    }
                }
            }
        }

        txt

    }
}