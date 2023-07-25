use colorful::{Color, Colorful};

use crate::game::{unit::Id, board::{Phase, Board, turn::ChooseTie}, skill::skill_list::tie::{Tie, TieWay}};

use super::{Choose, Return, CtrlPara};

use std::fmt::Write;

impl Board {
    pub fn turn_tie(&mut self, para : &mut CtrlPara, id : Id, it : Id, bound_point : i32) -> Return {

        let mut show = String::new();
        let sh = &mut show;
        
        // 生成选择
        let actor = self.get_unit(id);
        let target = self.get_unit(it);
        writeln!(sh, "捆绑 {} 的选择 (剩余捆绑点 = {}) : \n",target.identity() , bound_point.to_string().color(Color::Yellow)).unwrap();

        let mut choose = vec!(ChooseTie::Pass);
        writeln!(sh, "[{:^3}] : {}", 0, "放弃捆绑").unwrap();
        let mut count = 1;
        
        for (bound, is_tie) in Tie::new().tie_choose(actor, target) {
            match is_tie {
                TieWay::Tight => {
                    // [ 1 ] : 扎紧 [@--@---@](80%) 脚腕<-->后颈 (消耗捆绑点 = 20)
                    write!(sh, "[{:^3}] : {} {}{} {}", count, "扎紧".to_string().color(Color::Yellow), target.bound_identity(Some((&bound, true)), false), target.identity_tightness(&bound), bound.name()).unwrap();
                    match Tie::new().tight_get_cost_or_rate(bound_point, &bound, target) {
                        Ok(cost) => writeln!(sh, " (消耗捆绑点 = {})", cost.to_string().color(Color::Yellow)).unwrap(),
                        Err(hit) => writeln!(sh, " (消耗全部捆绑点，成功率 : {}%)", hit.to_string().color(Color::Yellow)).unwrap(),
                    }
                    choose.push(ChooseTie::Tight(bound));
                },
                TieWay::Tie => {
                    // [ 2 ] : 捆绑 [@O-@---@] 大臂 (消耗捆绑点 : 100)
                    write!(sh, "[{:^3}] : {} {} {}", count, "捆绑".to_string().color(Color::Green), target.bound_identity(Some((&bound, true)), false), bound.name()).unwrap();
                    match Tie::new().tie_get_cost_or_rate(bound_point, &bound, actor, target) {
                        Ok(cost) => writeln!(sh, " (消耗捆绑点 : {})", cost.to_string().color(Color::Yellow)).unwrap(),
                        Err(hit) => writeln!(sh, " (消耗全部捆绑点，成功率 : {}%)", hit.to_string().color(Color::Yellow)).unwrap(),
                    }
                    choose.push(ChooseTie::Tie(bound));
                },
                TieWay::Untie => {
                    // [ 7 ] : 解绑 [@--@---@] 手腕 (消耗捆绑点 : 100)
                    write!(sh, "[{:^3}] : {} {}{} {}", count, "解绑".to_string().color(Color::Red), target.bound_identity(Some((&bound, false)), false), target.identity_tightness(&bound), bound.name()).unwrap();
                    match Tie::new().untie_get_cost_or_rate(bound_point, &bound, target) {
                        Ok(cost) => writeln!(sh, " (消耗捆绑点 : {})", cost.to_string().color(Color::Yellow)).unwrap(),
                        Err(hit) => writeln!(sh, " (消耗全部捆绑点，成功率 : {}%)", hit.to_string().color(Color::Yellow)).unwrap(),
                    }
                    choose.push(ChooseTie::Untie(bound));
                },
            }
            count += 1;
        }

        // 默认选择
        // 优先加固（除非到解绑阶段）

        let ai_choose = if let Some(chic) = choose.get(1) {
            if let ChooseTie::Tight(_) = chic {
                chic.clone()
            }else{
                match target.ai_tie_choice() {
                    Some((bound, is_tie)) => match is_tie {
                        true => ChooseTie::Tie(bound),
                        false => ChooseTie::Untie(bound),
                    },
                    None => ChooseTie::Pass,
                }
            }
        }else{
            choose[0].clone()
        };

        // 分支，如果是玩家，返回行动，否则自动选择行动执行
        if actor.is_human() && !para.force_auto{
            writeln!(sh, "{}", "请选择 : ".to_string().color(Color::Yellow)).unwrap();
            if let Some(printer) = para.printer.as_mut() {
                printer.temp = show;
            }

            // 只有一个选项时自动选择
            if choose.len() == 1 {
                self.response_choose(para, Choose::Tie(choose[0].clone()))
            }else{
                Return::new_with_choose_and_default(
                    choose.into_iter().map(|a| Choose::Tie(a)).collect(),
                    Choose::Tie(ai_choose),
                )
            }
        }else{
            self.response_tie(para, ai_choose)
        }
    }

    pub fn response_tie(&mut self, para : &mut CtrlPara, choose : ChooseTie) -> Return {
        let mut str = String::new();
        let s = &mut str;
        if let Phase::Tie { id, it, bound_point } = self.phase {
            write!(s, "- ").unwrap();
            let remain = match choose {
                ChooseTie::Pass => {
                    writeln!(s, "放弃捆绑 (剩余点数 : {})", bound_point.to_string().color(Color::Yellow)).unwrap();
                    0
                },
                ChooseTie::Tight(bound) => {
                    Tie::new().exe_tight(s, bound, bound_point, self, it)
                },
                ChooseTie::Tie(bound) => {
                    Tie::new().exe_tie(s, bound, bound_point, self, id, it)
                },
                ChooseTie::Untie(bound) => {
                    Tie::new().exe_untie(s, bound, bound_point, self, it)
                },
            };
            
            if remain > 0 {
                self.phase = Phase::Tie { id, it, bound_point : remain };
            }else{
                Tie::new().end(s, self, id, it);
                self.phase = Phase::Main { id , can_wait: true };
            }
            if let Some(printer) = para.printer.as_mut() {
                printer.cache += &str;
            }
            self.continue_turn(para)
        }else{
            unreachable!();
        }
    }
}