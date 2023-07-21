use colorful::{Color, Colorful};

use crate::game::skill::Skill;

use super::{Unit, Id, bound::{BoundState, BoundPart}, Dir, Pos};

impl Unit {
    pub fn new_noal(id : Id, pos : Pos) -> Self {
        let color = Color::DodgerBlue3;
        let name = "诺艾尔".to_string().color(color).to_string();
        let name_fix_length = "诺艾尔  ".to_string().color(color).to_string();
        Self::new(id, name, name_fix_length, true, true, pos, 10, 10, 10)
    }

    pub fn new_noal_bound(id : Id, pos : Pos) -> Self {
        let color = Color::DodgerBlue3;
        let name = "诺艾尔".to_string().color(color).to_string();
        let name_fix_length = "诺艾尔  ".to_string().color(color).to_string();
        
        let mut noal = Self::new(id, name, name_fix_length, true, true, pos, 10, 10, 10);
        noal.tie(&BoundPart::Neck);
        noal.tie(&BoundPart::Arm);
        noal.tie(&BoundPart::Wrist);
        noal.tie(&BoundPart::Thigh);
        noal.tie(&BoundPart::Calve);
        noal.tie(&BoundPart::Ankle);
        noal.tie(&BoundPart::Long);
        noal.take_fall();
        noal
    }


    pub fn new_yelin(id : Id, pos : Pos) -> Self {
        let color = Color::Aquamarine1a;
        let name = "叶琳".to_string().color(color).to_string();
        let name_fix_length = "叶琳    ".to_string().color(color).to_string();
        Self::new(id, name, name_fix_length, true, false, pos, 16, 12, 14)

    }

    pub fn new_kuinuo(id : Id, pos : Pos) -> Self {
        let color = Color::LightSalmon3b;
        let name = "奎诺".to_string().color(color).to_string();
        let name_fix_length = "奎诺    ".to_string().color(color).to_string();
        let mut unit = Self::new(id, name, name_fix_length, false, false, pos, 16, 20, 18);
        unit.take_sleep();
        unit
    }
}

impl Unit {
    pub fn new(
        id : Id, 
        name : String, 
        name_fix_length : String,
        ally : bool,
        you : bool,
        pos : Pos,
        str : i32,
        dex : i32,
        agi : i32,
    ) -> Self {
        Self {
            id,
            name,
            name_fix_length,
            ally,
            you, 
            pos,
            str_max: str,
            dex_max: dex,
            agi_max: agi,
            inj_coefficient: 5,
            restore_rate: 25,
            bound: BoundState::new(),
            fall: false,
            stun: false,
            sleep: false,
            inj: 0,
            dir: match ally {
                true => Dir::Right,
                false => Dir::Left,
            },
            action: true,
            catch_left: None,
            catch_right: None,
            catched_left: None,
            catched_right: None,
            skills: Skill::basic_set(),
        }
    }
}