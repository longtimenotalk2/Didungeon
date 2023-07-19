use num_rational::Ratio;

use super::{Unit, Id, Dir, bound::BoundPart};

impl Unit {
    pub fn awake(&mut self) -> bool {
        if self.sleep {
            self.sleep = false;
            true
        }else{
            false
        }
    }

    pub fn recover_stun(&mut self) {
        self.stun = false;
    }

    pub fn take_dmg(&mut self, dmg : i32) -> i32 {
        self.inj += dmg;
        self.awake();
        self.inj
    }

    pub fn take_stun(&mut self) {
        self.stun = true;
        self.fall = true;
        self.action = false;
    }

    pub fn take_sleep(&mut self) {
        self.sleep = true;
        self.fall = true;
        self.action = false;
    }

    pub fn take_fall(&mut self) {
        self.fall = true;
    }

    pub fn catch_with(&mut self, id : Id, dir : &Dir) {
        match dir {
            Dir::Left => self.catch_left = Some(id),
            Dir::Right => self.catch_right = Some(id),
        }
    }

    pub fn catched_with(&mut self, id : Id, dir : &Dir) {
        match dir {
            Dir::Left => self.catched_right = Some(id),
            Dir::Right => self.catched_left = Some(id),
        }
    }

    pub fn cancel_catch_with(&mut self, id : Id) {
        if let Some(i) = self.catch_left {
            if i == id {
                self.catch_left = None;
            }
        }
        if let Some(i) = self.catch_right {
            if i == id {
                self.catch_right = None;
            }
        }
    }

    pub fn cancel_catched_with(&mut self, id : Id) {
        if let Some(i) = self.catched_left {
            if i == id {
                self.catched_left = None;
            }
        }
        if let Some(i) = self.catched_right {
            if i == id {
                self.catched_right = None;
            }
        }
    }

    pub fn check_to_stand(&mut self) -> bool {
        if self.can_stand() && !self.is_catched(){
            if self.fall {
                self.fall = false;
                return true;
            }
        }
        false
    }

    pub fn end_action(&mut self) {
        self.action = false;
    }

    pub fn end_turn(&mut self) {
        self.end_turn_restore();
        if !self.is_sleep() {
            self.action = true;
        }
    }

    fn end_turn_restore(&mut self) {
        let rate = Ratio::new(self.restore_rate, 100);
        let heal = rate * Ratio::from_integer(self.inj);
        let heal = heal.ceil().to_integer();
        self.inj -= heal;
    }

    pub fn tie(&mut self, bound : &BoundPart) {
        self.bound.tie(bound);
    }

    pub fn untie(&mut self, bound : &BoundPart) {
        self.bound.untie(bound);
    }

    pub fn tightness_change_to(&mut self, bound : &BoundPart, num : i32) {
        self.bound.tightness_change_to(bound, num);
    }
}