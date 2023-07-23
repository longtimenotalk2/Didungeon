use crate::game::unit::{Id, Dir, Pos};

use super::Board;

impl Board {
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

    fn pos_valid(&self, pos : Pos) -> bool {
        pos >= self.pos_min && pos < (self.pos_min + self.pos_length)
    }

    // 寻找相邻的角色
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

    // 寻找合法的移动格
    pub fn find_dest_with_range(&self, id : Id, range : i32) -> Vec<(Pos, Dir)> {
        let mut list = vec!();
        let pos = self.get_pos(id);
        let target_is_enemy = self.get_unit(id).get_ally();

        for dir in Dir::all() {
            for dx in 1..(range+1) {
                let pos = match dir {
                    Dir::Left => pos - dx,
                    Dir::Right => pos + dx,
                };
                if self.pos_valid(pos) {
                    let it = self.get_id_from_pos(pos);
                    if let Some(it) = it {
                        let target = self.get_unit(it);
                        if target.get_ally() == !target_is_enemy && target.can_block() {
                            break;
                        }else{
                            list.push((pos, dir.clone()));
                        }
                    }else{
                        list.push((pos, dir.clone()));
                    }
                }
            }
        }

        list
    }

    // 寻找不被敌方阻挡的，范围内的敌方角色
    pub fn find_target_with_range(&self, id : Id, range : i32) -> Vec<(Id, Dir)> {
        let mut list = vec!();
        let pos = self.get_pos(id);
        let target_is_enemy = self.get_unit(id).get_ally();

        for dir in Dir::all() {
            for dx in 1..(range+1) {
                let pos = match dir {
                    Dir::Left => pos - dx,
                    Dir::Right => pos + dx,
                };
                let it = self.get_id_from_pos(pos);
                if let Some(it) = it {
                    let target = self.get_unit(it);
                    if target.get_ally() == !target_is_enemy {
                        list.push((it, dir.clone()));
                        if target.can_block() {
                            break;
                        }
                    }
                }
            }
        }

        list
    }
}