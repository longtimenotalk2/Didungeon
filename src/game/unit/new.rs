use super::{Unit, Id, bound::BoundState, Dir, Pos};

impl Unit {
    pub fn new(
        id : Id, 
        name : String, 
        ally : bool,
        pos : Pos,
    ) -> Self {
        Self {
            id,
            name,
            ally,
            pos,
            str_max: 10,
            dex_max: 10,
            agi_max: 10,
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
        }
    }
}