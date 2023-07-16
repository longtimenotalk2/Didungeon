use crate::{game::unit::{Id, Dir, Pos, Unit}, wyrand::Dice};

use super::Board;

impl Board {
    pub fn get_dice(&mut self) -> &mut Dice {
        &mut self.dice
    }

    pub fn get_unit(&self, id : Id) -> &Unit {
        let i = self.indexs.get(&id).unwrap();
        &self.units[*i]
    }

    pub fn get_unit_mut(&mut self, id : Id) -> &mut Unit {
        let i = self.indexs.get(&id).unwrap();
        self.units.get_mut(*i).unwrap()
    }

    pub fn get_pos(&self, id : Id) -> Pos {
        self.get_unit(id).get_pos()
    }

    pub fn get_id_from_pos(&self, pos : Pos) -> Option<Id> {
        for unit in &self.units {
            if pos == unit.get_pos() {
                return Some(unit.get_id());
            }
        }
        None
    }

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
}