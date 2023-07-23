use colorful::{Color, Colorful};

use crate::{game::{board::{Board, Phase}, unit::Id, skill::skill_list::tie::Tie}, common::CLEAR};

use super::Return;
use std::fmt::Write;

impl Board {
    pub fn set_to_start(&mut self) {
        self.phase = Phase::Start;
    }

    pub fn turn_start(&mut self, need_show : bool) -> Return {

        // 找当前可动的速度最快的角色行动，如没有则进入下一回合
        let mut ido: Option<u32> = self.find_next_actor();
        if let None = ido {
            self.next_turn();
            self.acted_ids = vec![];
            ido = self.find_next_actor();
        }

        // 清除 Cache
        self.string_cache = CLEAR.to_string();
        let mut str = String::new();
        let s = &mut str;
        // 当前回合信息
        write!(s, "回合 : {}\n", self.turn).unwrap();
        // 已动角色信息
        *s += "行动次序 : ";
        for id in &self.acted_ids {
            *s += &self.get_unit(*id).identity().to_string();
            *s += " -> ";
        }
        // 生成回合人
        let id = ido.unwrap();
        *s += &self.get_unit(id).identity().to_string();
        *s += "\n";
        write!(s, "{} 的回合\n", self.get_unit(id).identity()).unwrap();

        //进入准备阶段
        self.phase = Phase::Prepare {id};

        self.string_cache += &str;
        self.continue_turn(need_show)
    }

    pub fn turn_prepare(&mut self, need_show : bool, id : Id) -> Return {
        // 根据当前是否处于擒拿状态，判断是否进入捆绑状态，或者直接进入主要阶段
        if let Some(it) = self.get_unit(id).get_catch() {
            let bound_point = Tie::new().bound_point(self.get_unit(id)); 
            self.phase = Phase::Tie { id, it, bound_point};
            // [捆绑] 诺艾尔 (捆绑点数 : 200)
            let target_idy = self.get_unit(it).identity();
            let s = &mut self.string_cache;
            writeln!(s, "[捆绑] {target_idy} (捆绑点数 : {})", bound_point.to_string().color(Color::Yellow)).unwrap();
            self.continue_turn(need_show)
        } else {
            self.phase = Phase::Auto { id };
            self.continue_turn(need_show)
        }
    }

    pub fn set_to_end(&mut self, id : Id) {
        self.phase = Phase::End {id};
    }

    pub fn turn_end(&mut self, need_show : bool, id : Id) -> Return {

        // 回合结束，进入下回合并按任意键继续
        self.get_unit_mut(id).end_action();
        if !self.acted_ids.contains(&id) {
            self.acted_ids.push(id);
        }

        if need_show {
            // 最终输出Cache
            println!("{}", self.string_cache);
            // 结果图
            self.show(None);
            println!();
        }
        

        // 判断胜负
        if let Some(a) = self.is_ally_win() {
            if need_show {
                match a {
                    true => println!("胜利"),
                    false => println!("失败"),
                }
            }
            
            Return::new_with_winner(a)
        }else{
            // 结束
            if need_show {
                println!("按任意键继续……");
            }
            
            Return::new()
        }
    
        
    }
}

