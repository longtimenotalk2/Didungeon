use colorful::{Color, Colorful};

use crate::game::{unit::Id, board::{Phase, Board, turn::ChooseUnbound}, skill::skill_list::unbound::Unbound};

use super::{Choose, Return};

use std::fmt::Write;

impl Board {
    pub fn set_to_unbound(&mut self, id : Id, bound_point : i32) {
        self.phase = Phase::Unbound { id, bound_point};
    }

    pub fn turn_unbound(&mut self, id : Id, bound_point : i32) -> Return {
        let mut show = String::new();
        let sh = &mut show;

        // 生成选择
        writeln!(sh, "解绑的选择 (剩余捆绑点 = {}) : \n", bound_point.to_string().color(Color::Yellow)).unwrap();

        let mut choose = vec!(ChooseUnbound::Pass);
        writeln!(sh, "[{:^3}] : {}", 0, "放弃解绑").unwrap();
        let mut count = 1;

        let actor = self.get_unit(id);
        for bound in Unbound::new().unbound_choice(actor) {
            // [ 7 ] : 解绑 [@--@---@] 手腕 (消耗捆绑点 : 100)
            write!(sh, "[{:^3}] : {} {}{} {}", count, "解绑".to_string().color(Color::Red), actor.bound_identity(Some((&bound, false)), false), actor.identity_tightness(&bound), bound.name()).unwrap();
            match Unbound::new().unbound_get_cost_or_rate(bound_point, &bound, actor) {
                Ok(cost) => writeln!(sh, " (消耗捆绑点 : {})", cost.to_string().color(Color::Yellow)).unwrap(),
                Err(hit) => writeln!(sh, " (消耗全部捆绑点，成功率 : {}%)", hit.to_string().color(Color::Yellow)).unwrap(),
            }
            choose.push(ChooseUnbound::Unbound(bound));
            count += 1;
        }

         // 分支，如果是玩家，返回行动，否则自动选择行动执行
         if actor.is_human() {
            println!();
            self.show(Some(id));
            println!();
            println!("{}", show);
            println!("{}", "请选择 : ".to_string().color(Color::Yellow));

            // 只有一个选项时自动选择
            if choose.len() == 1 {
                self.response_choose(Choose::Unbound(choose[0].clone()))
            }else{
                Return::new_with_choose(choose.into_iter().map(|a| Choose::Unbound(a)).collect())
            }
        }else{
            let choose = match actor.ai_unbound_choice() {
                Some(bound) => ChooseUnbound::Unbound(bound),
                None => ChooseUnbound::Pass,
            };
            self.response_unbound(choose)
        }
    }

    pub fn response_unbound(&mut self, choose : ChooseUnbound) -> Return {
        let mut str = String::new();
        let s = &mut str;

        if let Phase::Unbound { id, bound_point } = self.phase {
            write!(s, "- ").unwrap();
            let remain = match choose {
                ChooseUnbound::Pass => {
                    writeln!(s, "放弃解绑 (剩余点数 : {})", bound_point.to_string().color(Color::Yellow)).unwrap();
                    0
                },
                ChooseUnbound::Unbound(bound) => {
                    Unbound::new().exe_unbound(s, bound, bound_point, self, id)
                },
            };

            self.string_cache += &str;
            
            if remain > 0 {
                self.phase = Phase::Unbound { id, bound_point : remain };
                self.continue_turn()
            }else{
                // 自动起身
                if self.get_unit(id).is_fall() {
                    // [起身]
                    if self.get_unit_mut(id).check_to_stand() {
                        write!(&mut self.string_cache, "[起身]\n").unwrap();
                    } 
                }

                self.phase = Phase::End { id };

                self.continue_turn()
            }
            
            
        }else{
            unimplemented!()
        }

        
    }
}