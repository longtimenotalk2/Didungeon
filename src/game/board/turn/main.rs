

use colorful::{Color, Colorful};

use crate::game::{unit::Id, board::{Phase, Board, turn::ChooseSkill}, ai::AI};

use super::{Choose, Return, CtrlPara};

use std::fmt::Write;

impl Board {
    pub fn turn_main(&mut self, para : &mut CtrlPara, id : Id, can_wait : bool) -> Return {

        // 在缓存里存入当前场景
        if let Some(printer) = para.printer.as_mut() {
            printer.cache += "\n";
            printer.cache  += &self.txt_main_phase(id);
            printer.cache += "\n";
        }

        let mut show = String::new();
        let sh = &mut show;


        // 生成选择

        let actor = self.get_unit(id);
        let skills = actor.get_skills();

        writeln!(sh, "当前可选择的指令：").unwrap();

        let mut chooses = if can_wait && !actor.is_wait() {
            writeln!(sh, "  [{:^3}] : {}", 0, "等待").unwrap();
            vec!(ChooseSkill::Wait)
        }else{
            writeln!(sh, "  [{:^3}] : {}", 0, "跳过回合").unwrap();
            vec!(ChooseSkill::Pass)
        };

        let mut count = 1;
        // 技能选项
        for skill in skills {
            for (it, dir) in skill.get_targets(self, id) {
                writeln!(sh, "  [{:^3}] : {}", count, skill.choice_show(self, id, it, &dir)).unwrap();
                count += 1;
                chooses.push(ChooseSkill::Skill { skill: skill.clone(), it, dir });
            }
        }
        // 移动选项
        if !actor.is_fall() {
            let range = actor.move_range();
            for (pos, dir) in self.find_dest_with_range(id, range) {
                let move_abs = (pos - actor.get_pos()).abs();
                writeln!(sh, "  [{:^3}] : 移动 {} {}", count, dir.notice(), move_abs).unwrap();
                count += 1;
                chooses.push(ChooseSkill::Move { pos, dir});
            }
        }

        // 默认选择
        let ai_i = AI::new().analyse_skill(self, id, &chooses).unwrap();

        // 分支，如果是玩家，返回行动，否则自动选择行动执行

        let actor = self.get_unit(id);

        if actor.get_ally() && !para.force_auto {
            writeln!(sh, "{}", "请选择 : ".to_string().color(Color::Yellow)).unwrap();
            if let Some(printer) = para.printer.as_mut() {
                printer.temp = show;
            }

            // 只有一个选项时自动选择
            if chooses.len() == 1 {
                self.response_choose(para, Choose::Skill(chooses[0].clone()))
            }else{
                let choose = chooses[ai_i].clone();
                Return::new_with_choose_and_default(
                    chooses.into_iter().map(|a| Choose::Skill(a)).collect(), 
                    Choose::Skill(choose),
                )
            }
        }else{
            // AI自动按顺序选择
            let choose = chooses[ai_i].clone();
            self.response_main(para, choose)
        }
    }

    pub fn response_main(&mut self, para : &mut CtrlPara, choose : ChooseSkill) -> Return {

        if let Some(printer) = para.printer.as_mut() {
            printer.temp = String::new();
        }

        let mut str = String::new();
        let s = &mut str;
        if let Phase::Main {id, can_wait : _} = self.phase {
            match choose {
                ChooseSkill::Pass => {
                    writeln!(s, "跳过回合").unwrap();
                    writeln!(s).unwrap();
                    self.set_to_end(id);
                },
                ChooseSkill::Wait => {
                    writeln!(s, "等待").unwrap();
                    writeln!(s).unwrap();
                    self.set_to_wait(id);
                }
                ChooseSkill::Skill { skill, it, dir } => {
                    skill.exe(s, self, id, it, &dir);
                },
                ChooseSkill::Move { pos, dir } => {
                    let move_abs = (pos - self.get_pos(id)).abs();
                    writeln!(s, "移动 {} {}", dir.notice(), move_abs).unwrap();
                    *s += "\n";
                    self.actor_move_to(id, pos, dir);
                    self.check_awake(s);
                    self.set_to_end(id);
                },
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