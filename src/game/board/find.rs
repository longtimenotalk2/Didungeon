use crate::game::unit::{Id, Dir};

use super::Board;

impl Board {
    pub fn find_adjs(&self, id : Id) -> Vec<(Id, Dir)> {
        let pos = self.get_pos(id);
        let mut list = vec!();
        for (pos, dir) in [(pos-1, Dir::Left), (pos+1, Dir::Right)] {
            if let Some(id) = self.get_id_from_pos(pos) {
                list.push((id, dir));
            }
        }
        list
    }

    pub fn find_next_actor(&self) -> Option<Id> {
        let mut next : Option<(Id, i32)> = None;
        for unit in &self.units {
            if unit.is_action() {
                let id = unit.get_id();
                let spd = unit.spd();
                match next {
                    Some((_, s)) => {
                        if spd > s {
                            next = Some((id, spd));
                        }
                    },
                    None => {next = Some((id, spd));},
                }
            }
        }
        match next {
            Some((id, _)) => Some(id),
            None => None,
        }
    }
}