use colorful::Colorful;
use colorful::Color;

use crate::game::unit::Id;
use crate::game::unit::Unit;

use std::fmt::Write;

use super::Board;

const COLOR_STRONG: Color = Color::Red;
const COLOR_WEAK : Color = Color::Green;

fn remove0_color(num : i32, color : Option<Color>, special_type : Option<bool>) -> String {
    let a = if num > 0 {
        format!("{:^3}", num)
    }else{
        match special_type {
            Some(true) => format!(" {} ", "x".to_string().color(Color::Red).to_string()),
            Some(false) => format!(" {} ", "√".to_string().color(Color::Green).to_string()),
            None => "   ".to_string(),
        }
    };
    if let Some(color) = color {
        a.color(color).to_string()
    }else{
        a
    }
}

fn show_main_phase_attr(is_actor : bool, list : Vec<(i32, i32)>) -> String {
    let mut s = String::new();
    for (attr, base) in list {
        match is_actor {
            true => s += &remove0_color(attr, None, Some(true)),
            false => if attr < base {
                s += &remove0_color(attr, Some(COLOR_WEAK), Some(false));
            }else{
                s += &remove0_color(attr, Some(COLOR_STRONG), Some(false));
            },
        }
    }
    s
}

impl Board {
    pub fn txt_main_phase(&self, id : Id) -> String{
        let mut txt = String::new();
        // 标题
        let repeat = 28;
        txt += &" ".repeat(repeat);
        txt += "速 移 受 力 灵 敏 命 攻 压\n";
        txt += &" ".repeat(repeat);
        txt += "度 动 伤 量 巧 捷 回 防 挣\n";
        // 先清点当前行动者状态
        let actor = self.get_unit(id);
        let acc = actor.acc_melee_hand();
        let atk = actor.atk_melee_hand();
        let hld = actor.hold_force();
        let range = actor.move_range() + 1;

        let id_actor = id;

        // 下一个该行动的角色
        let next_id = self.find_next_actor_except(id_actor);
        
        for pos in self.pos_min .. (self.pos_min + self.pos_length) {
            if let Some(id) = self.get_id_from_pos(pos) {
                let unit = self.get_unit(id);
                let mut s = String::new();
                if id == id_actor{
                    s += "> ";
                    s += &unit.identity_wo_attr();
                    s += " ";
                    s += &unit.identity_basic_attr(true, false);

                    s += &show_main_phase_attr(true, vec![
                        (acc, acc),
                        (atk, atk),
                        (hld, hld),
                    ]);
                    
                }else{
                    s += "  ";
                    s += &unit.identity_wo_attr();
                    s += " ";
                    if next_id.is_some_and(|a| a == id) {
                        s += &unit.identity_basic_attr(false, true);
                    }else{
                        s += &unit.identity_basic_attr(false, false);
                    }

                    // 判断是否为敌方以及是否notice
                    if let Some(is_notice) = self.find_if_target_insight_return_if_notice(id_actor, id, range) {
                        let (evd, def) = match is_notice {
                            true => (unit.evd(), unit.def_gym()),
                            false => (unit.evd_back(), unit.def_gym_back()),
                        };
                        s += &show_main_phase_attr(false, vec![
                            (evd, acc),
                            (def, atk),
                            (unit.struggle_force(), hld),
                        ]);
                    }
                    
                }
                txt += &s;
                txt += "\n"
            }
            else {
                writeln!(&mut txt, "   {}", "<空>".to_string().color(Color::Grey0)).unwrap();
            }
        }
        txt
    }

    pub fn show(&self, actor : Option<Id>) {
        Unit::show_title1();
        Unit::show_title2();
        for pos in self.pos_min .. (self.pos_min + self.pos_length) {
            if let Some(id) = self.get_id_from_pos(pos) {
                let is_act = if let Some(id_actor) = actor {
                    if id_actor == id {
                        true
                    }else{
                        false
                    }
                }else{
                    false
                };
                self.get_unit(id).show(is_act)
            }else{
                println!("   {}", "<空>".to_string().color(Color::Grey0));
            }
        }
    }
}