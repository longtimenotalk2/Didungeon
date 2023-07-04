use std::ops::Index;

use super::Skillize;

pub struct Tie {
    weight_neck : i32,
    weight_arm : i32,
    weight_hang : i32,
    weight_wrist : i32,
    weight_joint : i32,
    weight_thigh : i32,
    weight_calve : i32,
    weight_ankle : i32,
    weight_long : i32,
}

impl Tie {
    pub fn new() -> Self {
        Self {
            weight_neck: 100,
            weight_arm: 100,
            weight_hang: 100,
            weight_wrist: 100,
            weight_joint: 100,
            weight_thigh: 100,
            weight_calve: 100,
            weight_ankle: 100,
            weight_long: 100,
        }
    }
}

impl Skillize for Tie {
    fn can(&self) -> Box<dyn Fn(&crate::game::board::Board, u8, Option<u8>) -> bool> {
        Box::new(
            |board, ia, ibo| {
                let a = board.index(ia);
                let b = board.index(ibo.unwrap());
                if !b.hold {return false};
                if a.bound_wrist {return false};
                b.next_can_tie_choices().len() > 0
            }
        )
    }

    fn evaluate(&self) -> Box<dyn Fn(&crate::game::board::Board, u8, Option<u8>) -> (i32, Option<String>) + '_> {
        todo!()
    }

    fn exe(&self) -> Box<dyn FnMut(&mut crate::game::board::Board, u8, Option<u8>, &mut crate::wyrand::Dice) -> String + '_> {
        todo!()
    }
}