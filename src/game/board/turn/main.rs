

use colorful::{Color, Colorful};

use crate::{game::{unit::Id, board::{Phase, Board, turn::ChooseSkill}}, common};

use super::Choose;

impl Board {
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