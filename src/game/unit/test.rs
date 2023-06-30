use super::Unit;

impl Unit {
    pub fn test_new1() -> Self {
        #[allow(warnings)]
        let bound_neck : bool = false;
        #[allow(warnings)]
        let bound_arm : bool = false;
        #[allow(warnings)]
        let bound_hang : bool = false;
        #[allow(warnings)]
        let bound_wrist : bool = false;
        #[allow(warnings)]
        let bound_joint : bool = false;
        #[allow(warnings)]
        let bound_thigh : bool = false;
        #[allow(warnings)]
        let bound_calve : bool = false;
        #[allow(warnings)]
        let bound_ankle : bool = false;
        #[allow(warnings)]
        let bound_long: bool = false;
        #[allow(warnings)]
        let fall : bool = false;

        // let bound_neck : bool = true;
        // let bound_arm : bool = true;
        // let bound_hang : bool = true;
        // let bound_wrist : bool = true;
        // let bound_joint : bool = true;
        // let bound_thigh : bool = true;
        // let bound_calve : bool = true;
        // let bound_ankle : bool = true;
        // let bound_long: bool = true;
        // let fall : bool = true;

        Self {
            str_max : 10,
            dex_max : 10,
            agi_max : 10,
            inj_decay_rate : 5,
            inj : 0,

            bound_neck,
            bound_arm,
            bound_hang,
            bound_wrist,
            bound_joint,
            bound_thigh,
            bound_calve,
            bound_ankle,
            bound_long,
            
            fall,
            hold : false,
            action : true,
            name : "a".to_string(),
            
        }
    }

    pub fn test_new2() -> Self {
        #[allow(warnings)]
        let bound_neck : bool = false;
        #[allow(warnings)]
        let bound_arm : bool = false;
        #[allow(warnings)]
        let bound_hang : bool = false;
        #[allow(warnings)]
        let bound_wrist : bool = false;
        #[allow(warnings)]
        let bound_joint : bool = false;
        #[allow(warnings)]
        let bound_thigh : bool = false;
        #[allow(warnings)]
        let bound_calve : bool = false;
        #[allow(warnings)]
        let bound_ankle : bool = false;
        #[allow(warnings)]
        let bound_long: bool = false;
        #[allow(warnings)]
        let fall : bool = false;

        // let bound_neck : bool = true;
        // let bound_arm : bool = true;
        // let bound_hang : bool = true;
        let bound_wrist : bool = true;
        // let bound_joint : bool = true;
        // let bound_thigh : bool = true;
        // let bound_calve : bool = true;
        let bound_ankle : bool = true;
        // let bound_long: bool = true;
        // let fall : bool = true;

        Self {
            str_max : 16,
            dex_max : 20,
            agi_max : 18,
            inj_decay_rate : 5,
            inj : 0,

            bound_neck,
            bound_arm,
            bound_hang,
            bound_wrist,
            bound_joint,
            bound_thigh,
            bound_calve,
            bound_ankle,
            bound_long,
            
            fall,
            hold : false,
            action : true,
            name : "a".to_string(),
            
        }
    }
}