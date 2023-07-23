use crate::game::unit::{Id, Pos, Dir};

use super::Board;

impl Board {
    pub fn actor_move_to(&mut self, id : Id, pos : Pos, dir : Dir) {
        let anti_dir = dir.anti();
        let adder = match anti_dir {
            Dir::Left => -1,
            Dir::Right => 1,
        };
        let mut dup_pos: i32 = pos;
        let mut list_anti_move = vec!{};
        while let Some(it) = self.get_id_from_pos(dup_pos) {
            if it != id {
                list_anti_move.push(it);
                dup_pos += adder;
            }
        }
        self.get_unit_mut(id).move_to(pos, dir);
        for id in list_anti_move {
            let pos = self.get_pos(id);
            self.get_unit_mut(id).move_to(pos + adder, anti_dir.clone())
        }
    }
}