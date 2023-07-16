use super::{Unit, Id, Dir};

impl Unit {
    pub fn set_catch(&mut self, id : Id, dir : Dir) {
        match dir {
            Dir::Left => self.catch_left = Some(id),
            Dir::Right => self.catch_right = Some(id),
        }
    }

    pub fn set_stun(&mut self, r : bool) {
        self.stun = r;
    }

    pub fn set_sleep(&mut self, r : bool) {
        self.sleep = r;
    }

    pub fn set_fall(&mut self, r : bool) {
        self.fall = r;
    }
}