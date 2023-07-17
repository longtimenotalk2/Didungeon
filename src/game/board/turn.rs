use colorful::Color;
use colorful::Colorful;

use crate::game::unit::bound::BoundPart;
use crate::{game::{skill::Skill, unit::{Id, Dir}}, common};

use super::Board;

#[derive(Clone, Debug)]
pub enum Command {
    Continue,
    Choose(Choose),
}

#[derive(Clone, Debug)]
pub enum Choose {
    Pass,
    Skill {skill : Skill, it : Id, dir : Dir,},
    Tie(BoundPart),
    Untie(BoundPart),
    PassTie,
}

impl Board {
    pub fn respond(&mut self, command : Command) -> Option<Vec<Choose>> {
        match command {
            Command::Continue => {
                if self.temp_remained_bound_value > 0 {
                    self.show(Some(self.temp_actor_now.unwrap()));
                    println!();
                    self.continue_tie()
                }else{
                    self.continue_turn()
                }
            }
            Command::Choose(choose) => {
                match choose {
                    Choose::Pass => {
                        self.exe_pass();
                        None
                    },
                    Choose::Skill { skill, it, dir } => {
                        self.exe_skill(skill, it, dir);
                        None
                    },
                    Choose::Tie(bound) => self.exe_tie(bound),
                    Choose::Untie(bound) => self.exe_untie(bound),
                    Choose::PassTie => {self.end_tie()},
                }
            },
        }
    }
}

impl Board {
    fn continue_turn(&mut self) -> Option<Vec<Choose>> {
        // 找当前可动的速度最快的角色行动
        let mut ido: Option<u32> = self.find_next_actor();
        if let None = ido {
            self.turn_end();
            ido = self.find_next_actor();
        }

        // 准备屏幕
        common::clear_cmd();
        println!("当前回合 : {}", self.turn);
        self.show(ido);
        println!();

        // 生成回合人
        let id = ido.unwrap();
        self.temp_actor_now = Some(id);
        let actor = self.get_unit(id);
        println!("{} 的回合", actor.identity());
        println!();

        // 自动行动
        match self.auto_action(id) {
            Some(chooses) => {
                Some(chooses)
            },
            None => self.create_choose(),
        }
    }

    pub fn create_choose(&mut self) -> Option<Vec<Choose>>{
        // 生成选择
        let id = self.temp_actor_now.unwrap();
        let actor = self.get_unit(id);
        let skills = actor.get_skills();
        
        let chooses = self.calc_chooses(id, skills);

        println!("当前可选择的指令：");
        
        let mut choose_count = 0;
        for choose in &chooses {
            match choose {
                Choose::Pass => {
                    println!("  [{:^3}] : {}", choose_count, "跳过回合");
                    choose_count += 1;
                },
                Choose::Skill { skill, it, dir } => {
                    println!("  [{:^3}] : {}{} -> {}", choose_count, skill.name(), dir.notice(), self.get_unit(*it).identity());
                    choose_count += 1;
                },
                _ => unreachable!(),
            }
        }
        println!();

        // 分支，如果是玩家，返回行动，否则自动选择行动执行
        if actor.is_human() {
            println!("{}", "请选择 : ".to_string().color(Color::Yellow));
            Some(chooses)
        }else{
            let choose = self.ai_choose(id, chooses);
            match choose {
                Choose::Pass => self.exe_pass(),
                Choose::Skill { skill, it, dir } => self.exe_skill(skill, it, dir),
                _ => unreachable!(),
            }
            None
        }
    }

    fn ai_choose(&self, _id : Id, chooses : Vec<Choose>) -> Choose {
        match chooses.get(1) {
            Some(c) => c.clone(),
            None => chooses.get(0).unwrap().clone(),
        }
    }

    pub fn exe_pass(&mut self) {
        let id = self.temp_actor_now.unwrap();
        println!("跳过回合");
        println!();

        self.get_unit_mut(id).end_action();
        self.temp_actor_now = None;
        // 结束
        println!("按任意键继续……")
    }

    pub fn exe_skill(&mut self, skill : Skill, it : Id, dir : Dir) {
        // 执行行动
        let id = self.temp_actor_now.unwrap();
        skill.exe(self, id, it, &dir);

        self.get_unit_mut(id).end_action();
        self.temp_actor_now = None;
        println!();

        // 结果图
        self.show(None);
        println!();

        // 结束
        println!("按任意键继续……")
    }

    fn turn_end(&mut self) {
        self.turn += 1;
        for unit in &mut self.units {
            unit.end_turn()
        }
    }

    fn calc_chooses(&self, id : Id, skills : &[Skill]) -> Vec<Choose> {
        let mut list = vec!(Choose::Pass);
        for skill in skills {
            for (it, dir) in skill.get_targets(self, id) {
                list.push(Choose::Skill { skill: skill.clone(), it, dir })
            }
        }
        list
    }
}