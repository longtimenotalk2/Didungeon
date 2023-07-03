use crate::game::board::Board;

impl Board {
    pub fn can_unbound(&self, ia : i32) -> bool {
        let a = self.index(ia);
        a.next_force_upper().is_some() || a.next_force_lower().is_some() || (!a.hold && a.next_hand().is_some())
    }

    pub fn can_tie(&self, ia : i32, ib : i32) -> bool {
        let a = self.index(ia);
        let b = self.index(ib);

        !a.bound_wrist && b.next_can_tie_choices().len() > 0 && b.hold
    }

    pub fn can_holddown(&self, ia : i32, ib : i32) -> bool {
        let a = self.index(ia);
        let b = self.index(ib);
        if a.fall {return false;}
        if b.hold {return false;}
        true
    }

    pub fn can_struggle(&self, ia : i32) -> bool {
        let a = self.index(ia);
        a.hold
    }

    pub fn can_auto_stand(&self, ia : i32) -> bool {
        let a = self.index(ia);
        a.fall && !a.hold && a.can_stand()
    } 
}