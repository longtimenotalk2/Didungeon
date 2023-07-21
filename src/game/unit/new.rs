use crate::game::skill::Skill;

use super::{Unit, Id, bound::BoundState, Dir, Pos};

impl Unit {
    pub fn new_noal(id : Id, pos : Pos) -> Self {
        Self::new(id, "诺艾尔".to_string(), true, true, pos, 10, 10, 10)
    }

    pub fn new_yelin(id : Id, pos : Pos) -> Self {
        Self::new(id, "叶琳".to_string(), true, false, pos, 15, 12, 14)
    }

    pub fn new_kuinuo(id : Id, pos : Pos) -> Self {
        let mut unit = Self::new(id, "奎诺".to_string(), false, false, pos, 16, 20, 18);
        unit.take_sleep();
        unit
    }
}

impl Unit {
    pub fn new(
        id : Id, 
        name : String, 
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