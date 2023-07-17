use crate::game::skill::Skill;

use super::{Unit, Id, Dir, Pos, bound::BoundPart};

impl Unit {
    pub fn get_id(&self) -> Id {
        self.id
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }

    pub fn get_inj(&self) -> i32 {
        self.inj
    }

    pub fn get_skills(&self) -> &[Skill] {
        &self.skills
    }

    pub fn is_human(&self) -> bool {
        self.you
    }

    pub fn get_catch_with(&self) -> Option<Id> {
        self.catch_left.or(self.catch_right)
    }

    pub fn can_tie_list(&self) -> Vec<BoundPart> {
        self.bound.can_tie_list()
    }

    pub fn can_untie_list(&self) -> Vec<BoundPart> {
        self.bound.can_untie_list()
    }
}

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

    pub fn set_action(&mut self, r : bool) {
        self.action = r;
    }

    
}