

use colorful::{Color, Colorful};

use crate::{game::{unit::Id, board::{Phase, Board, turn::{ChooseSkill, ChooseTie}}, skill::skill_list::tie::{Tie, TieWay}}, common};

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

    pub fn turn_main(&mut self, id : Id) -> Option<Vec<Choose>> {
        // 生成选择
        common::clear_cmd();
        println!("当前回合 = {}", self.turn);
        self.show(Some(id));
        println!();

        let actor = self.get_unit(id);
        let skills = actor.get_skills();

        println!("当前可选择的指令：");

        let mut chooses = vec!(ChooseSkill::Pass);
        println!("  [{:^3}] : {}", 0, "跳过回合");
        let mut count = 1;
        for skill in skills {
            for (it, dir) in skill.get_targets(self, id) {
                println!("  [{:^3}] : {}{} -> {}", count, skill.name(), dir.notice(), self.get_unit(it).identity());
                count += 1;
                chooses.push(ChooseSkill::Skill { skill: skill.clone(), it, dir });
            }
        }

        // 分支，如果是玩家，返回行动，否则自动选择行动执行

        if actor.is_human() {
            println!("{}", "请选择 : ".to_string().color(Color::Yellow));
            Some(chooses.into_iter().map(|a| Choose::Skill(a)).collect())
        }else{
            // AI自动按顺序选择
            let choose = match chooses.get(1) {
                Some(_) => chooses[1].clone(),
                None => chooses[0].clone(),
            };
            self.response_main(choose)
        }
    }

    pub fn response_main(&mut self, choose : ChooseSkill) -> Option<Vec<Choose>> {
        if let Phase::Main {id} = self.phase {
            match choose {
                ChooseSkill::Pass => {
                    println!("跳过回合");
                    println!();
                },
                ChooseSkill::Skill { skill, it, dir } => {
                    skill.exe(self, id, it, &dir);
                },
            }
            self.get_unit_mut(id).end_action();
            self.phase = Phase::Start;
            // 结果图
            self.show(None);
            println!();
            // 结束
            println!("按任意键继续……");
            None
        }else{
            unreachable!();
        }
        
    }

    
}