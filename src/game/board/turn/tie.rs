use colorful::{Color, Colorful};

use crate::game::{unit::Id, board::{Phase, Board, turn::ChooseTie}, skill::skill_list::tie::{Tie, TieWay}};

use super::Choose;

impl Board {
    pub fn turn_tie(&mut self, id : Id, it : Id, bound_point : i32) -> Option<Vec<Choose>> {
        
        // 生成选择
        println!("当前可选择的捆绑指令(剩余捆绑点 = {}) : ", bound_point.to_string().color(Color::Yellow));

        let mut choose = vec!(ChooseTie::Pass);
        println!("  [{:^3}] : {}", 0, "放弃捆绑");
        let mut count = 1;
        let actor = self.get_unit(id);
        let target = self.get_unit(it);

        for (bound, is_tie) in Tie::new().tie_choose(target) {
            match is_tie {
                TieWay::Tight => {
                    print!("  [{:^3}] : {} {} {}{} {}", count, "加固".to_string().color(Color::Yellow), target.identity(), target.bound_identity_change(&bound, true), target.identity_tightness(&bound), bound.name());
                    match Tie::new().tight_get_cost_or_rate(bound_point, &bound, target) {
                        Ok(cost) => println!(" (消耗捆绑点 = {})", cost.to_string().color(Color::Yellow)),
                        Err(hit) => println!(" (消耗全部捆绑点，成功率 = {}%)", hit.to_string().color(Color::Yellow)),
                    }
                    choose.push(ChooseTie::Untie(bound));
                },
                TieWay::Tie => {
                    print!("  [{:^3}] : 捆绑 {} {} {}", count, target.identity(), target.bound_identity_change(&bound, true), bound.name());
                    match Tie::new().tie_get_cost_or_rate(bound_point, &bound, actor, target) {
                        Ok(cost) => println!(" (消耗捆绑点 = {})", cost.to_string().color(Color::Yellow)),
                        Err(hit) => println!(" (消耗全部捆绑点，成功率 = {}%)", hit.to_string().color(Color::Yellow)),
                    }
                    choose.push(ChooseTie::Tie(bound));
                },
                TieWay::Untie => {
                    print!("  [{:^3}] : 解绑 {} {}{} {}", count, target.identity(), target.bound_identity_change(&bound, false), target.identity_tightness(&bound), bound.name());
                    match Tie::new().untie_get_cost_or_rate(bound_point, &bound, target) {
                        Ok(cost) => println!(" (消耗捆绑点 = {})", cost.to_string().color(Color::Yellow)),
                        Err(hit) => println!(" (消耗全部捆绑点，成功率 = {}%)", hit.to_string().color(Color::Yellow)),
                    }
                    choose.push(ChooseTie::Untie(bound));
                },
            }
            count += 1;
        }

        // 分支，如果是玩家，返回行动，否则自动选择行动执行
        if actor.is_human() {
            println!("{}", "请选择 : ".to_string().color(Color::Yellow));
            Some(choose.into_iter().map(|a| Choose::Tie(a)).collect())
        }else{
            let choose = match target.ai_tie_choice() {
                Some((bound, is_tie)) => match is_tie {
                    true => ChooseTie::Tie(bound),
                    false => ChooseTie::Untie(bound),
                },
                None => ChooseTie::Pass,
            };
            self.response_tie(choose, true)
        }
    }

    pub fn response_tie(&mut self, choose : ChooseTie, stop : bool) -> Option<Vec<Choose>> {
        if let Phase::Tie { id, it, bound_point } = self.phase {
            let remain = match choose {
                ChooseTie::Pass => {
                    Tie::new().exe_pass(self, id, it);
                    0
                },
                ChooseTie::Tight(_) => todo!(),
                ChooseTie::Tie(bound) => {
                    Tie::new().exe_tie(bound, bound_point, self, id, it)
                },
                ChooseTie::Untie(bound) => {
                    Tie::new().exe_untie(bound, bound_point, self, id, it)
                },
            };
            if remain > 0 {
                self.phase = Phase::Tie { id, it, bound_point : remain };
                if stop {
                    println!("按任意键继续……");
                    None
                } else {
                    self.continue_turn()
                }
            }else{
                self.phase = Phase::Main { id };
                // 结束
                println!("按任意键继续……");
                None
            }
        }else{
            unreachable!();
        }
    }
}