use crate::game::{unit::Unit, skill::txt_hit};

use super::{BASIC_HIT, HIT_RATE, to_hit, Skillize};

pub struct Struggle {
    basic_hit : i32,
    hit_rate : i32,
    auto : bool,
}

impl Struggle {
    pub fn new_auto() -> Self {
        Self {
            basic_hit: BASIC_HIT,
            hit_rate: HIT_RATE,
            auto : true,
        }
    }

    pub fn new() -> Self {
        Self {
            basic_hit: BASIC_HIT + 100,
            hit_rate: HIT_RATE,
            auto : false,
        }
    }

    fn can(&self, a : &Unit) -> bool {
        a.fall && a.can_stand()
    }

    fn hit(&self, a : &Unit, bs : &[&Unit]) -> i32 {
        let mut evd = 0;
        for b in bs {
            if a.ally != b.ally {
                evd += b.hold();
            }
        }
        let acc = a.anti_hold();

        let hit = to_hit(self.basic_hit + (acc - evd) * self.hit_rate);
        hit
    }
}

impl Skillize for Struggle {
    fn target(&self, board : &crate::game::board::Board, ia : u8) -> Vec<u8> {
        let a = board.index(ia);
        if self.can(a) {
            vec!(ia)
        }else{
            vec!()
        }
    }

    fn evaluate(&self, board : &crate::game::board::Board, ia : u8, _ib : u8) -> (i32, Option<String>) {
        let a = board.index(ia);
        let ibs = board.find_adjacent(ia);
        let mut bs = vec!();
        for ib in ibs {
            bs.push(board.index(ib))
        }
        let hit = self.hit(a, &bs);
        (hit, None)
    }

    fn exe(&self, board : &mut crate::game::board::Board, ia : u8, _ib : u8, dice : &mut crate::wyrand::Dice) -> String {
        let mut txt = String::new();
        
        let a = board.index(ia);
        let ibs = board.find_adjacent(ia);
        let mut bs = vec!();
        for ib in ibs {
            bs.push(board.index(ib))
        }
        let hit = self.hit(a, &bs);

        txt += if self.auto {"<auto stand>"} else {"<struggle>"};

        let hit_dice = dice.d(100);
        let is_hit = hit >= hit_dice;
        txt += &format!("{}", txt_hit("struggle", hit, hit_dice, is_hit, "success"));
        if is_hit {
            let a = board.index_mut(ia);
            a.fall = false;
        }
        txt
    }
}