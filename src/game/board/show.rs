use colorful::Colorful;
use colorful::Color;

use crate::game::unit::Id;
use crate::game::unit::Unit;

use super::Board;

impl Board {
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