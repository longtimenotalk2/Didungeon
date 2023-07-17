use colorful::Color;
use colorful::Colorful;

use crate::{game::{skill::Skill, unit::{Id, Dir}}, common};

use super::Board;

pub enum Command {
    Continue,
    Pass,
    Choose(Choose),
}

#[derive(Clone)]
pub struct Choose {
    skill : Skill,
    it : Id,
    dir : Dir,
}

impl Board {
    pub fn respond(&mut self, command : Command) -> Option<Vec<Choose>> {
        match command {
            Command::Continue => self.continue_turn(),
            Command::Pass => {
                self.exe_choose(self.actor.unwrap(), None);
                None
            },
            Command::Choose(c) => {
                self.exe_choose(self.actor.unwrap(), Some(c));
                None
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
        self.actor = Some(id);
        let actor = self.get_unit(id);
        println!("{} 的回合", actor.identity());
        println!();

        // 自动行动
        if self.auto_action(id) {
            println!();
            self.show(ido);
            println!();
        }

        // 生成选择
        let actor = self.get_unit(id);
        let skills = actor.get_skills();
        
        let chooses = self.calc_chooses(id, skills);

        println!("当前可选择的指令：");
        println!("  [{:^3}] : {}", 0, "跳过回合");
        let mut choose_count = 1;
        for choose in &chooses {
            let Choose {skill, it, dir} = choose;
            println!("  [{:^3}] : {}{} -> {}", choose_count, skill.name(), dir.notice(), self.get_unit(*it).identity());
            choose_count += 1;
        }
        println!();

        // 分支，如果是玩家，返回行动，否则自动选择行动执行
        if actor.is_human() {
            println!("{}", "请选择 : ".to_string().color(Color::Yellow));
            Some(chooses)
        }else{
            let choose = self.ai_choose(id, chooses);
            self.exe_choose(id, choose);
            None
        }
    }

    fn auto_action(&mut self, id : Id) -> bool {
        // 自动起身
        if self.get_unit_mut(id).check_to_stand() {
            println!("[自动起身]");
            return true;
        }
        false
    }

    fn ai_choose(&self, _id : Id, chooses : Vec<Choose>) -> Option<Choose>{
        chooses.get(0).map(|a| a.clone())
    }

    pub fn exe_choose(&mut self, id : Id, choose : Option<Choose>) {
        // 执行行动
        match choose {
            Some(Choose {skill, it, dir}) => skill.exe(self, id, it, &dir),
            None => println!("跳过回合"),
        }
        self.get_unit_mut(id).end_action();
        self.actor = None;
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
        let mut list = vec!();
        for skill in skills {
            for (it, dir) in skill.get_targets(self, id) {
                list.push(Choose { skill: skill.clone(), it, dir })
            }
        }
        list
    }
}