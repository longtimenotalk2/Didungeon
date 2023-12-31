use super::{Unit, bound::BoundPart};

impl Unit {
    pub fn is_stun(&self) -> bool {
        self.stun
    }

    pub fn is_sleep(&self) -> bool {
        self.sleep
    }

    pub fn is_able(&self) -> bool {
        !(self.is_stun() || self.is_sleep())
    }

    pub fn is_fall(&self) -> bool {
        self.fall
    }

    pub fn is_bound(&self, part : &BoundPart) -> bool {
        self.bound.is_bound(part)
    }

    pub fn is_action(&self) -> bool {
        self.action
    }

    pub fn free_upper(&self) -> bool {
        self.bound.free_upper()
    }

    pub fn free_lower(&self) -> bool {
        self.bound.free_lower()
    }

    pub fn is_bound_bow(&self) -> bool {
        self.bound.is_bound_bow()
    }

    pub fn is_defeated(&self) -> bool {
        self.bound.is_defeated()
    }

    pub fn can_block(&self) -> bool {
        !self.fall
    }
}