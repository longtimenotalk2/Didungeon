use crate::{game::{board::{Board, Phase}, unit::Id, skill::skill_list::tie::Tie}, common};

use super::Choose;

impl Board {
    pub fn turn_start(&mut self) -> Option<Vec<Choose>> {
        // 找当前可动的速度最快的角色行动，如没有则进入下一回合
        let mut ido: Option<u32> = self.find_next_actor();
        if let None = ido {
            self.turn_end();
            ido = self.find_next_actor();
        }

        // 生成回合人
        let id = ido.unwrap();
        self.phase = Phase::Prepare {id};
        self.continue_turn()
    }

    pub fn turn_prepare(&mut self, id : Id) -> Option<Vec<Choose>> {
        // 根据当前是否处于擒拿状态，判断是否进入捆绑状态，或者直接进入主要阶段
        if let Some(it) = self.get_unit(id).get_catch_with() {
            let bound_point = Tie::new().bound_point(self.get_unit(id)); 
            self.phase = Phase::Tie { id, it, bound_point};
            common::clear_cmd();
            println!("当前回合 = {}", self.turn);
            self.show(Some(id));
            println!();
            self.continue_turn()
        } else {
            self.phase = Phase::Auto { id };
            self.continue_turn()
        }
    }
}