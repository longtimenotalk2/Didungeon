use crate::game::unit::{Id, Pos, Dir};

use super::Board;

impl Board {
    pub fn actor_move_to(&mut self, id : Id, pos : Pos, dir : Dir) {
        let anti_dir = dir.anti();
        let adder = match anti_dir {
            Dir::Left => -1,
            Dir::Right => 1,
        };

        // 穿过受惊
        let pos0 = self.get_unit(id).get_pos();
        let mut consider_pos = pos;
        while consider_pos != pos0 {
            if let Some(it) = self.get_id_from_pos(consider_pos) {
                self.get_unit_mut(it).shock();
            }
            consider_pos += adder;
        }
        
        let mut dup_pos: i32 = pos;
        let mut list_anti_move = vec!{};
        while let Some(it) = self.get_id_from_pos(dup_pos) {
            if it != id {
                list_anti_move.push(it);
                dup_pos += adder;
            }else{
                break;
            }
        }
        self.get_unit_mut(id).move_to(pos, dir);
        for id in list_anti_move {
            let pos = self.get_pos(id);
            self.get_unit_mut(id).move_to(pos + adder, anti_dir.clone())
        }
    }
}