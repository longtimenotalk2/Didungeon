use crate::game::unit::{Id, Pos, Dir};

use super::Board;

impl Board {
    pub fn actor_move_to(&mut self, id : Id, pos : Pos, dir : Dir) {
        let anti_dir = dir.anti();
        let adder = match anti_dir {
            Dir::Left => -1,
            Dir::Right => 1,
        };

        // 被移动到的目标解除catch与catched
        if let Some(id) = self.get_id_from_pos(pos) {
            let target = self.get_unit(id);
            if let Some(it) = target.get_catch_with_dir(&dir) {
                self.get_unit_mut(id).cancel_catch_with(it);
                self.get_unit_mut(it).cancel_catched_with(id);
            }
            let target = self.get_unit(id);
            if let Some(it) = target.get_catched_with_dir(&dir) {
                self.get_unit_mut(id).cancel_catched_with(it);
                self.get_unit_mut(it).cancel_catch_with(id);
            }
        }

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
        self.get_unit_mut(id).move_to(pos, &dir);
        for id in list_anti_move {
            let pos = self.get_pos(id);
            self.get_unit_mut(id).move_to(pos + adder, &dir)
        }
    }

    pub fn dash_to(&mut self, id : Id, it : Id, dir : &Dir) {
        let pos = self.get_unit(it).get_pos();
        let dest = pos + match dir {
            Dir::Left => 1,
            Dir::Right => -1,
        };
        if dest != self.get_unit(id).get_pos() {
            self.actor_move_to(id, dest, dir.clone())
        }else{
            self.get_unit_mut(id).move_to(dest, &dir)
        }
    }
}