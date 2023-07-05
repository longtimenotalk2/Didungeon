use crate::game::{unit::Unit, skill::txt_hit};

use super::{BASIC_HIT, HIT_RATE, to_hit, Skillize, txt_announce, Skill};

pub struct Struggle {
    basic_hit : i32,
    hit_rate : i32,
}

impl Struggle {
    pub fn new() -> Self {
        Self {
            basic_hit: BASIC_HIT,
            hit_rate: HIT_RATE,
        }
    }

    fn can(&self, a : &Unit, b : &Unit, ia : u8) -> bool {
        if let Some(iaa) = b.catch {
            if iaa == ia {
                return a.anti_hold() > 0
            }
        }
        false
    }

    fn hit(&self, a : &Unit, b : &Unit) -> i32 {
        let acc = a.anti_hold();
        let evd = b.hold();
        let hit = to_hit(self.basic_hit + (acc - evd) * self.hit_rate);
        hit
    }
}

impl Skillize for Struggle {
    fn can(&self, board : &crate::game::board::Board, ia : u8, ib : u8) -> bool {
        let a = board.index(ia);
        let b = board.index(ib);
        self.can(a, b, ia)
    }

    fn evaluate(&self, board : &crate::game::board::Board, ia : u8, ib : u8) -> (i32, Option<String>) {
        let a = board.index(ia);
        let b = board.index(ib);
        (self.hit(a, b), None)
    }

    fn exe(&self, board : &mut crate::game::board::Board, ia : u8, ib : u8, dice : &mut crate::wyrand::Dice) -> String {
        let mut txt = String::new();
        
        let a = board.index(ia);
        let b = board.index(ib);

        txt += &txt_announce(&Skill::Struggle, ib);

        let hit = self.hit(a, b);
        let hit_dice = dice.d(100);
        let is_hit = hit >= hit_dice;
        txt += &format!("{}", txt_hit("struggle", hit, hit_dice, is_hit, "success"));
        if is_hit {
            let mut a = board.index_mut(ia);
            a.hold = false;
            if a.fall && a.can_stand() {
                a.fall = false;
                txt += &format!("<stand auto>\n");
            }
        }
        txt
    }
}