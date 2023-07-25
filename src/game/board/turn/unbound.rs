use colorful::{Color, Colorful};

use crate::game::{unit::Id, board::{Phase, Board, turn::ChooseUnbound}, skill::skill_list::unbound::Unbound};

use super::{Choose, Return, CtrlPara};

use std::fmt::Write;

impl Board {
    pub fn set_to_unbound(&mut self, id : Id, bound_point : i32) {
        self.phase = Phase::Unbound { id, bound_point};
    }

    pub fn turn_unbound(&mut self, para : &mut CtrlPara, id : Id, bound_point : i32) -> Return {
        let mut show = String::new();
        let sh = &mut show;

        // 生成选择
        writeln!(sh, "\n脱缚的选择 (剩余捆绑点 = {}) : \n", bound_point.to_string().color(Color::Yellow)).unwrap();

        let mut chooses = vec!(ChooseUnbound::Pass);
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
            chooses.push(ChooseUnbound::Unbound(bound));
            count += 1;
        }

        // 默认选择
        let choose = match actor.ai_unbound_choice() {
            Some(bound) => ChooseUnbound::Unbound(bound),
            None => ChooseUnbound::Pass,
        };

        // 分支，如果是玩家，返回行动，否则自动选择行动执行
        if actor.is_human() && !para.force_auto{
            writeln!(sh, "{}", "请选择 : ".to_string().color(Color::Yellow)).unwrap();
            if let Some(printer) = para.printer.as_mut() {
                printer.temp = show;
            }

            // 只有一个选项时自动选择
            if chooses.len() == 1 {
                self.response_choose(para, Choose::Unbound(chooses[0].clone()))
            }else{
                Return::new_with_choose_and_default(
                    chooses.into_iter().map(|a| Choose::Unbound(a)).collect(),
                    Choose::Unbound(choose),
                )
            }
        }else{
            self.response_unbound(para, choose)
        }
    }

    pub fn response_unbound(&mut self, para : &mut CtrlPara,  choose : ChooseUnbound) -> Return {
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

            if let Some(printer) = para.printer.as_mut() {
                printer.cache += &str;
            }
            
            if remain > 0 {
                self.phase = Phase::Unbound { id, bound_point : remain };
                self.continue_turn(para)
            }else{
                // 自动起身
                if self.get_unit(id).is_fall() {
                    // [起身]
                    if self.get_unit_mut(id).check_to_stand() {
                        if let Some(printer) = para.printer.as_mut() {
                            write!(printer.cache, "[起身]\n").unwrap()
                        }
                        ;
                    } 
                }

                self.phase = Phase::End { id };

                self.continue_turn(para)
            }
            
            
        }else{
            unimplemented!()
        }

        
    }
}