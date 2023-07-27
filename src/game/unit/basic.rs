use crate::game::skill::Skill;

use super::{Unit, Id, Dir, Pos, bound::BoundPart};

impl Unit {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> Id {
        self.id
    }
    
    pub fn get_ally(&self) -> bool {
        self.ally
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }

    pub fn get_dir(&self) -> Dir {
        self.dir.clone()
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

    pub fn get_catch(&self) -> Option<Id> {
        self.catch_left.or(self.catch_right)
    }

    pub fn get_catched(&self) -> Vec<Id> {
        let mut list = vec!();
        if let Some(it) = self.catched_left {
            list.push(it);
        }
        if let Some(it) = self.catched_right {
            list.push(it);
        }
        list
    }

    pub fn get_catch_with_dir(&self, dir : &Dir) -> Option<Id> {
        match dir {
            Dir::Left => self.catch_left,
            Dir::Right => self.catch_right,
        }
    }

    pub fn get_catched_with_dir(&self, dir : &Dir) -> Option<Id> {
        match dir {
            Dir::Left => self.catched_left,
            Dir::Right => self.catched_right,
        }
    }

    pub fn is_catched(&self) -> bool {
        self.catched_left.is_some() || self.catched_right.is_some()
    }

    pub fn is_wait(&self) -> bool {
        self.wait
    }

    pub fn can_tie_list(&self) -> Vec<BoundPart> {
        self.bound.can_tie_list()
    }

    pub fn can_untie_list(&self) -> Vec<BoundPart> {
        self.bound.can_untie_list()
    }

    pub fn get_tightness(&self, part : &BoundPart) -> i32 {
        self.bound.get_tightness(part)
    }

    pub fn ai_tie_choice(&self) -> Option<(BoundPart, bool)> {
        self.bound.ai_tie_choice()
    }

    pub fn ai_unbound_choice(&self) -> Option<BoundPart> {
        self.bound.ai_unbound_choice()
    }

    pub fn ai_untie_choice(&self) -> Option<BoundPart> {
        self.bound.ai_untie_choice()
    }

    pub fn next_force_upper(&self) -> Option<BoundPart> {
        self.bound.next_force_upper()
    }

    pub fn next_force_lower(&self) -> Option<BoundPart> {
        self.bound.next_force_lower()
    }

    pub fn has_bound(&self) -> bool{
        self.bound.has_bound()
    }

    pub fn is_notice(&self, dir : &Dir) -> bool {
        if self.fall {return false};
        self.dir != *dir
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