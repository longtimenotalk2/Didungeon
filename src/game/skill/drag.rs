use crate::game::unit::Unit;

use super::{Skillize, txt_announce, Skill};

pub struct Drag {

}

impl Drag {
    pub fn new() -> Self {
        Self {

        }
    }
    
    fn can(&self, a : &Unit, b : &Unit) -> bool {
        if a.bound_wrist {return false;}
        if !b.fall {(return false);}
        true
    }

    fn hit(&self, a : &Unit, b : &Unit) -> i32 {
        if a.push() > b.anti_hold() {
            100
        }else{
            0
        }
    }
}

impl Skillize for Drag {
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

    fn evaluate(&self, board : &crate::game::board::Board, ia : u8, ib : u8) -> (i32, Option<String>) {
        let a = board.index(ia);
        let b = board.index(ib);
        let hit = self.hit(a, b);

        (hit, None)
    }

    fn exe(&self, board : &mut crate::game::board::Board, ia : u8, ib : u8, dice : &mut crate::wyrand::Dice) -> String {
        let mut txt = String::new();

        txt += &txt_announce(&Skill::Drag, ib);

        // rush
        let arr = board.rush_to(ia, ib);

        let a = board.index(ia);
        let b = board.index(ib);
        let hit = self.hit(a, b);

        let hit_dice = dice.d(100);
        let is_hit = hit >= hit_dice;

        if is_hit {
            
        }

        txt
    }
}