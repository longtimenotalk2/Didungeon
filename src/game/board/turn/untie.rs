use colorful::{Color, Colorful};

use crate::game::{board::{Board, Phase, turn::Choose}, unit::Id, skill::skill_list::untie::Untie};

use super::{Return, ChooseUntie, CtrlPara};

use std::fmt::Write;

impl Board {
    pub fn set_to_untie(&mut self, id : Id, it : Id,  bound_point : i32) {
        self.phase = Phase::Untie { id, it, bound_point};
    }

    pub fn turn_untie(&mut self, para : CtrlPara, id : Id, it : Id, bound_point : i32) -> Return {
        let mut show = String::new();
        let sh = &mut show;

        // 生成选择
        writeln!(sh, "解绑的选择 (剩余捆绑点 = {}) : \n", bound_point.to_string().color(Color::Yellow)).unwrap();

        let mut choose = vec!(ChooseUntie::Pass);
        writeln!(sh, "[{:^3}] : {}", 0, "放弃解绑").unwrap();
        let mut count = 1;

        
        let target = self.get_unit(it);
        for bound in Untie::new().untie_choice(target) {
            // [ 7 ] : 解绑 [@--@---@] 手腕 (消耗捆绑点 : 100)
            write!(sh, "[{:^3}] : {} {}{} {}", count, "解绑".to_string().color(Color::Red), target.bound_identity(Some((&bound, false)), false), target.identity_tightness(&bound), bound.name()).unwrap();
            match Untie::new().unbound_get_cost_or_rate(bound_point, &bound, target) {
                Ok(cost) => writeln!(sh, " (消耗捆绑点 : {})", cost.to_string().color(Color::Yellow)).unwrap(),
                Err(hit) => writeln!(sh, " (消耗全部捆绑点，成功率 : {}%)", hit.to_string().color(Color::Yellow)).unwrap(),
            }
            choose.push(ChooseUntie::Untie(bound));
            count += 1;
        }

         // 分支，如果是玩家，返回行动，否则自动选择行动执行
         let actor = self.get_unit(id);
         if actor.is_human() && !para.force_auto{
            println!("{}", show);
            println!("{}", "请选择 : ".to_string().color(Color::Yellow));

            // 只有一个选项时自动选择
            if choose.len() == 1 {
                self.response_choose(para, Choose::Untie(choose[0].clone()))
            }else{
                Return::new_with_choose(choose.into_iter().map(|a| Choose::Untie(a)).collect())
            }
        }else{
            let choose = match target.ai_untie_choice() {
                Some(bound) => ChooseUntie::Untie(bound),
                None => ChooseUntie::Pass,
            };
            self.response_untie(para, choose)
        }
    }

    pub fn response_untie(&mut self, para : CtrlPara,  choose : ChooseUntie) -> Return {
        let mut str = String::new();
        let s = &mut str;

        if let Phase::Untie { id, it, bound_point } = self.phase {
            write!(s, "- ").unwrap();
            let remain = match choose {
                ChooseUntie::Pass => {
                    writeln!(s, "放弃解绑 (剩余点数 : {})", bound_point.to_string().color(Color::Yellow)).unwrap();
                    0
                },
                ChooseUntie::Untie(bound) => {
                    Untie::new().exe_untie(s, bound, bound_point, self, it)
                },
            };

            self.string_cache += &str;
            
            if remain > 0 {
                self.phase = Phase::Untie { id, it, bound_point : remain };
                self.continue_turn(para)
            }else{
                self.phase = Phase::End { id };

                self.continue_turn(para)
            }
        }else{
            unimplemented!()
        }
    }
}