use colorful::{Color, Colorful};

use crate::{game::{board::{Board, Phase}, unit::Id, skill::skill_list::tie::Tie}, common::CLEAR};

use super::Return;
use std::fmt::Write;

impl Board {
    pub fn turn_start(&mut self) -> Return {
        // 判断胜负

        // 找当前可动的速度最快的角色行动，如没有则进入下一回合
        let mut ido: Option<u32> = self.find_next_actor();
        if let None = ido {
            self.turn_end();
            ido = self.find_next_actor();
        }

        // 清除 Cache
        self.string_cache = CLEAR.to_string();
        let mut str = String::new();
        let s = &mut str;
        // 当前回合信息
        write!(s, "回合 : {}\n", self.turn).unwrap();
        // 生成回合人
        let id = ido.unwrap();
        write!(s, "{} 的回合\n", self.get_unit(id).identity()).unwrap();

        //进入准备阶段
        self.phase = Phase::Prepare {id};

        self.string_cache += &str;
        self.continue_turn()
    }

    pub fn turn_prepare(&mut self, id : Id) -> Return {
        // 根据当前是否处于擒拿状态，判断是否进入捆绑状态，或者直接进入主要阶段
        if let Some(it) = self.get_unit(id).get_catch_with() {
            let bound_point = Tie::new().bound_point(self.get_unit(id)); 
            self.phase = Phase::Tie { id, it, bound_point};
            // [捆绑] 诺艾尔 (捆绑点数 : 200)
            let target_idy = self.get_unit(it).identity();
            let s = &mut self.string_cache;
            writeln!(s, "[捆绑] {target_idy} (捆绑点数 : {})", bound_point.to_string().color(Color::Yellow)).unwrap();
            self.continue_turn()
        } else {
            self.phase = Phase::Auto { id };
            self.continue_turn()
        }
    }
}