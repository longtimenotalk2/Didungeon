

use colorful::{Color, Colorful};

use crate::game::{unit::Id, board::{Phase, Board, turn::ChooseSkill}};

use super::{Choose, Return};

use std::fmt::Write;

impl Board {
    pub fn turn_main(&mut self, id : Id) -> Return {
        let mut show = String::new();
        let sh = &mut show;
        // 生成选择

        let actor = self.get_unit(id);
        let skills = actor.get_skills();

        writeln!(sh, "当前可选择的指令：").unwrap();

        let mut chooses = vec!(ChooseSkill::Pass);
        writeln!(sh, "  [{:^3}] : {}", 0, "跳过回合").unwrap();
        let mut count = 1;
        for skill in skills {
            for (it, dir) in skill.get_targets(self, id) {
                writeln!(sh, "  [{:^3}] : {}", count, skill.choice_show(self, id, it, &dir)).unwrap();
                count += 1;
                chooses.push(ChooseSkill::Skill { skill: skill.clone(), it, dir });
            }
        }

        // 分支，如果是玩家，返回行动，否则自动选择行动执行

        if actor.is_human() {
            println!();
            self.show(Some(id));
            println!();
            println!("{}", show);
            println!("{}", "请选择 : ".to_string().color(Color::Yellow));

            // 只有一个选项时自动选择
            if chooses.len() == 1 {
                self.response_choose(Choose::Skill(chooses[0].clone()))
            }else{
                Return {
                    choose: Some(chooses.into_iter().map(|a| Choose::Skill(a)).collect()),
                }
            }
        }else{
            // AI自动按顺序选择
            let choose = match chooses.get(1) {
                Some(_) => chooses[1].clone(),
                None => chooses[0].clone(),
            };
            self.response_main(choose)
        }
    }

    pub fn response_main(&mut self, choose : ChooseSkill) -> Return {
        let mut str = String::new();
        let s = &mut str;
        if let Phase::Main {id} = self.phase {
            match choose {
                ChooseSkill::Pass => {
                    writeln!(s, "跳过回合").unwrap();
                    writeln!(s).unwrap();
                    self.set_to_end(id);
                },
                ChooseSkill::Skill { skill, it, dir } => {
                    skill.exe(s, self, id, it, &dir);
                },
            }
            
            self.string_cache += &str;

            self.continue_turn()
        }else{
            unreachable!();
        }
        
    }

    
}