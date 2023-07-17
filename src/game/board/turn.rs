use crate::{game::{skill::Skill, unit::{Id, Dir}}, common};

use super::Board;

pub enum Command {
    Continue,
}

struct Choose {
    skill : Skill,
    it : Id,
    dir : Dir,
}

impl Board {
    pub fn respond(&mut self, command : Command) {
        match command {
            Command::Continue => self.continue_turn(),
        }
    }
}

impl Board {
    fn continue_turn(&mut self) {
        // 找当前可动的速度最快的角色行动
        let mut id: Option<u32> = self.find_next_actor();
        if let None = id {
            self.turn_end();
            id = self.find_next_actor();
        }

        // 准备屏幕
        common::clear_cmd();
        self.show(id);
        println!();

        // 生成回合人
        let id = id.unwrap();
        let actor = self.get_unit(id);
        println!("{} 的回合", actor.identity());
        println!();

        // 生成选择
        let skills = actor.get_skills();
        
        let chooses = self.calc_chooses(id, skills);

        println!("当前可选择的指令");
        let mut choose_count = 0;
        for choose in &chooses {
            let Choose {skill, it, dir} = choose;
            println!("  [{:^3}] : {}{} -> {}", choose_count, skill.name(), dir.notice(), self.get_unit(*it).identity());
            choose_count += 1;
        }
        println!();

        // 执行行动
        match chooses.get(0) {
            Some(Choose {skill, it, dir}) => skill.exe(self, id, *it, dir),
            None => println!("无法行动"),
        }
        self.get_unit_mut(id).end_action();
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
            unit.set_action(true);
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