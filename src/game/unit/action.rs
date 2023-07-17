use num_rational::Ratio;

use super::Unit;

impl Unit {
    pub fn take_dmg(&mut self, dmg : i32) -> i32 {
        self.inj += dmg;
        self.sleep = false;
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

    pub fn check_to_stand(&mut self) -> bool {
        if self.can_stand() {
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
}