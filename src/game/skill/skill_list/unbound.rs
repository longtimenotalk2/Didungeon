use std::vec;

use crate::game::{skill::{helper, Skillize}, unit::{Unit, Dir, Id, bound::BoundPart}, board::Board};

pub struct Unbound {
    basic_hit : i32,
    hit_rate : i32,
}

impl Unbound {
    pub fn new() -> Self {
        Self {
            basic_hit: 0,
            hit_rate: 20,
        }
    }

    pub fn hit(&self, actor : &Unit) -> i32 {
        let acc = actor.hand_dex();
        helper::to_hit(self.basic_hit + self.hit_rate * acc)
    }

    pub fn unbound_choice(&self, actor : &Unit) -> Vec<BoundPart> {
        actor.can_untie_list()
    }

    pub fn unbound_get_cost_or_rate(&self, bound_point : i32, bound : &BoundPart, actor : &Unit) -> Result<i32, i32> {
        let ramain_tightness = actor.get_tightness(bound);
        let cost = ramain_tightness;
        if cost < bound_point {
            return Ok(cost)
        }else{
            return Err(100 + bound_point - ramain_tightness)
        }
    }
}

impl Skillize for Unbound {
    fn get_targets(&self, board : &Board, id : Id) -> Vec<(Id, Dir)> {
        let actor = board.get_unit(id);
        if actor.has_bound() && self.hit(actor) > 0 {
            vec![(id, actor.get_dir())]
        }else{
            vec![]
        }
    }

    fn exe(&self, s : &mut String, board : &mut Board, id : Id, _it : Id, _dir : &Dir) {
        let actor = board.get_unit(id);
        let bound_point = self.hit(actor);
        board.set_to_unbound(id, bound_point);
    }
    
}