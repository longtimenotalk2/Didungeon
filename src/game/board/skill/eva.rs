use crate::game::{board::Board, unit::Bound};

use super::to_hit;

impl Board {
    pub fn hit_unbound(&self, ia : i32) -> i32 {
        let mut hit_now = 0;
        let a = self.index(ia);
        if let Some(_) = a.next_force_upper() {
            let hit = to_hit(a.unbound_force_upper() * 5);
            hit_now = hit_now.max(hit);
        };
        if let Some(_) = a.next_force_lower() {
            let hit = to_hit(a.unbound_force_lower() * 5);
            hit_now = hit_now.max(hit);
        };
        if !self.index(ia).hold {
            if let Some(_) = a.next_hand() {
                let hit = to_hit(a.hand_dex() * 5);
                hit_now = hit_now.max(hit);
            }
        }
        if !a.fall && a.bound_hang {
            let hit = to_hit(a.spd() * 5);
            hit_now = hit_now.max(hit);
        }
        
        hit_now
    }

    pub fn choice_tie(&self, ia : i32, ib : i32) -> Option<(Bound, bool, i32)> {
        let a = self.index(ia);
        let b = self.index(ib);
        let choices =b.next_can_tie_choices();
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

    pub fn hit_holddown(&self, ia : i32, ib : i32) -> (i32, i32, i32) {
        let a = self.index(ia);
        let b = self.index(ib);
        let acc1 = a.acc_melee_hand();
        let evd1 = b.evd();
        let hit1 = to_hit(50 + (acc1 - evd1) * 5);
        let acc2 = a.push();
        let evd2 = b.anti_push();
        let mut hit2 = to_hit(50 + (acc2 - evd2) * 5);
        if self.index(ib).fall {
            hit2 = 100;
        }
        let acc3 = a.hold();
        let evd3 = b.anti_hold();
        let hit3 = to_hit(50 + (acc3 - evd3) * 5    );
        (hit1, hit2, hit3)
    }

    pub fn hit_struggle(&self, ia : i32, ib : i32) -> i32 {
        let a = self.index(ia);
        let b = self.index(ib);
        let acc = a.anti_hold();
        let evd = b.hold();
        to_hit(50 + (acc - evd) * 5)
    }
}