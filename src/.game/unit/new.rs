use super::Unit;

impl Unit {
    pub fn new(
        str : i32,
        dex : i32,
        agi : i32,
        ally : bool,
    ) -> Self {
        Self {
            str_max: str,
            dex_max: dex,
            agi_max: agi,
            inj_coefficient: 5,
            restore_rate: 25,
            inj: 0,
            bound_neck: false,
            bound_arm: false,
            bound_hang: false,
            bound_wrist: false,
            bound_joint: false,
            bound_thigh: false,
            bound_calve: false,
            bound_ankle: false,
            bound_long: false,
            fall: false,
            stun: false,
            action: true,
            ally,
        }
    }

    pub fn new_blank(ally : bool) -> Self {
        Self::new(15, 15, 15, ally)
    }
}