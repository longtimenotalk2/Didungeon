use super::Unit;

impl Unit {
    pub fn take_dmg(&mut self, dmg : i32) -> i32 {
        self.inj += dmg;
        self.inj
    }

    pub fn take_stun(&mut self) {
        self.stun = true;
        self.fall = true;
        self.action = false;
    }

    pub fn end_action(&mut self) {
        self.action = false;
    }
}