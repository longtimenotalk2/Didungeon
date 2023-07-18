use crate::game::{board::{Board, Phase}, unit::Id};

use super::Choose;

impl Board {
    pub fn turn_auto(&mut self, id : Id) -> Option<Vec<Choose>> {
        // 自动起身
        let mut need_fresh = false;
        if self.get_unit_mut(id).check_to_stand() {
            println!("[自动起身]");
            need_fresh = true;
        }

        if need_fresh {
            self.show(Some(id));
            println!();
            println!("按任意键继续……");
            return None;
        }

        self.phase = Phase::Main {id};

        self.continue_turn()
    }
}