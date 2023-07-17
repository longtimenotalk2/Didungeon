use colorful::Color;
use colorful::Colorful;

use crate::game::{unit::{Id, bound::BoundPart}, skill::skill_list::tie::Tie};

use super::{Board, turn::Choose};

impl Board {
    pub fn auto_action(&mut self, id : Id) -> Option<Vec<Choose>> {
        // 自动起身
        if self.get_unit_mut(id).check_to_stand() {
            println!("[自动起身]");
            println!();
            self.show(Some(id));
            println!();
        }
        // 捆绑开始
        if let Some(it) = self.get_unit(id).get_catch_with() {
            self.temp_actor_now = Some(id);
            self.temp_target_now = Some(it);
            self.temp_remained_bound_value = Tie::new().bound_num(self.get_unit(id));
            return self.continue_tie();
        }
        None
    }

    pub fn continue_tie(&self) -> Option<Vec<Choose>> {
        let mut list = vec!(Choose::PassTie);
        let target = self.get_unit(self.temp_target_now.unwrap());
        for (bound, is_tie) in Tie::new().tie_choose(target) {
            match is_tie {
                true => list.push(Choose::Tie(bound)),
                false => list.push(Choose::Untie(bound)),
            }
        }

        let bound_num = self.temp_remained_bound_value;
        let actor = self.get_unit(self.temp_actor_now.unwrap());
        let target = self.get_unit(self.temp_target_now.unwrap());
        println!("当前可选择的捆绑指令(剩余捆绑点 = {}) : ", bound_num.to_string().color(Color::Yellow));
        let mut choose_count = 0;
        for choose in &list {
            match choose {
                Choose::PassTie => {
                    println!("  [{:^3}] : {}", choose_count, "放弃捆绑");
                    choose_count += 1;
                },
                Choose::Tie(bound) => {
                    print!("  [{:^3}] : 捆绑 {} ",choose_count, target.identity());
                    target.show_bound_with_change(bound, true);
                    print!(" {}", bound.name_tie());
                    match Tie::new().tie_get_cost_or_rate(bound_num, bound, actor, target) {
                        Ok(cost) => println!(" (消耗捆绑点 = {})", cost.to_string().color(Color::Yellow)),
                        Err(hit) => println!(" (消耗全部捆绑点，成功率 = {}%)", hit.to_string().color(Color::Yellow)),
                    }
                    choose_count += 1;
                },
                Choose::Untie(bound) => {
                    print!("  [{:^3}] : 解绑 {} ",choose_count, target.identity());
                    target.show_bound_with_change(bound, false);
                    print!(" {}", bound.name_untie());
                    match Tie::new().untie_get_cost_or_rate(bound_num, bound, target) {
                        Ok(cost) => println!(" (消耗捆绑点 = {})", cost.to_string().color(Color::Yellow)),
                        Err(hit) => println!(" (消耗全部捆绑点，成功率 = {}%)", hit.to_string().color(Color::Yellow)),
                    }
                    choose_count += 1;
                }
                _ => unreachable!(),
            }
        }

        Some(list)
    }

    pub fn exe_tie(&mut self, bound : BoundPart) -> Option<Vec<Choose>> {
        let id = self.temp_actor_now.unwrap();
        let it = self.temp_target_now.unwrap();
        let remain = Tie::new().exe_tie(bound, self.temp_remained_bound_value, self, id, it);
        self.temp_remained_bound_value = remain;
        if remain == 0 {
            self.temp_target_now = None;
            self.create_choose()
        }else{
            self.continue_tie()
        }
    }

    pub fn exe_untie(&mut self, bound : BoundPart) -> Option<Vec<Choose>> {
        let id = self.temp_actor_now.unwrap();
        let it = self.temp_target_now.unwrap();
        let remain = Tie::new().exe_untie(bound, self.temp_remained_bound_value, self, id, it);
        self.temp_remained_bound_value = remain;
        if remain == 0 {
            self.temp_target_now = None;
            self.create_choose()
        }else{
            self.continue_tie()
        }
    }

    pub fn end_tie(&mut self) -> Option<Vec<Choose>> {
        println!("放弃捆绑");
        println!();
        self.temp_target_now = None;
        self.temp_remained_bound_value = 0;
        self.create_choose();

        todo!()
    }
}