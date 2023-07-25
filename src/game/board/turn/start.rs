use colorful::{Color, Colorful};

use crate::game::{board::{Board, Phase}, unit::Id, skill::skill_list::tie::Tie};

use super::{Return, CtrlPara};
use std::fmt::Write;

impl Board {
    pub fn set_to_start(&mut self) {
        self.phase = Phase::Start;
    }

    pub fn turn_start(&mut self, para : &mut CtrlPara) -> Return {

        // 找当前可动的速度最快的角色行动，如没有则进入下一回合
        let mut ido: Option<u32> = self.find_next_actor();
        if let None = ido {
            self.next_turn();
            self.acted_ids = vec![];
            ido = self.find_next_actor();
        }

        // 清除 Cache
        if let Some(printer) = para.printer.as_mut() {
            printer.cache = String::new();
        }


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

        if let Some(printer) = para.printer.as_mut() {
            printer.cache += &str;
        }
        self.continue_turn(para)
    }

    pub fn turn_prepare(&mut self, para : &mut CtrlPara, id : Id) -> Return {
        // 根据当前是否处于擒拿状态，判断是否进入捆绑状态，或者直接进入主要阶段
        if let Some(it) = self.get_unit(id).get_catch() {
            let bound_point = Tie::new().bound_point(self.get_unit(id)); 
            self.phase = Phase::Tie { id, it, bound_point};
            // [捆绑] 诺艾尔 [...] (捆绑点数 : 200)
            let target_idy = self.get_unit(it).identity();
            let bound_idy = self.get_unit(it).bound_identity(None, true);
            if let Some(printer) = para.printer.as_mut() {
                writeln!(printer.cache, "[捆绑] {target_idy} {bound_idy} (捆绑点数 : {})", bound_point.to_string().color(Color::Yellow)).unwrap();
            }
            
            self.continue_turn(para)
        } else {
            self.phase = Phase::Auto { id };
            self.continue_turn(para)
        }
    }

    pub fn set_to_wait(&mut self, id : Id) {
        self.phase = Phase::Wait { id }
    }

    pub fn turn_wait(&mut self, para : &mut CtrlPara, id : Id) -> Return {
        // 等待，进入下回合并按任意键继续
        self.get_unit_mut(id).wait();

        // 判断胜负
        if let Some(a) = self.is_ally_win() {
            
            if let Some(printer) = para.printer.as_mut() {
                printer.cache += match a {
                    true => "胜利\n",
                    false => "失败\n",
                };
            }
            
            Return::new_with_winner(a)
        }else{
            // 结束
            if let Some(printer) = para.printer.as_mut() {
                printer.cache += "按任意键继续……";
            }
            self.set_to_start();
            Return::new()
        }
    }

    pub fn set_to_end(&mut self, id : Id) {
        self.phase = Phase::End {id};
    }

    pub fn turn_end(&mut self, para : &mut CtrlPara, id : Id) -> Return {

        // 回合结束，进入下回合并按任意键继续
        self.get_unit_mut(id).end_action();
        if !self.acted_ids.contains(&id) {
            self.acted_ids.push(id);
        }

        // 判断胜负
        if let Some(a) = self.is_ally_win() {
            if let Some(printer) = para.printer.as_mut() {
                printer.cache += match a {
                    true => "胜利\n",
                    false => "失败\n",
                };
            }
            
            Return::new_with_winner(a)
        }else{
            // 结束
            if let Some(printer) = para.printer.as_mut() {
                printer.cache += "按任意键继续……";
            }
            
            self.set_to_start();
            Return::new()
        }
    }
}

