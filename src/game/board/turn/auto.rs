use colorful::{Color, Colorful};

use crate::game::{board::{Board, Phase}, unit::Id, skill::skill_list::{struggle::Struggle, force_unbound::ForceUnbound}};

use super::Choose;

impl Board {
    pub fn turn_auto(&mut self, id : Id) -> Option<Vec<Choose>> {
        let mut need_fresh = false;
        // 自动挣扎
        if self.get_unit(id).is_catched() {
            need_fresh = true;
            Struggle::new().exe(self, id);
        }

        // 自动挣脱
        if self.get_unit(id).has_bound() {
            need_fresh = true;
            ForceUnbound::new().exe(self, id)
        }

        // 自动起身
        if self.get_unit(id).is_fall() {
            need_fresh = true;
            if self.get_unit_mut(id).check_to_stand() {
                println!("[尝试起身] {}", "成功".to_string().color(Color::Green));
            } else {
                println!("[尝试起身] {}", "失败".to_string().color(Color::Red));
            }
        }

        self.phase = Phase::Main {id};
        
        if need_fresh {
            self.show(Some(id));
            println!();
            println!("按任意键继续……");
            return None;
        }
        self.continue_turn()
    }
}