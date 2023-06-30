use super::Unit;

impl Unit {
    pub fn test_new1() -> Self {
        Self {
            str_max : 20,
            dex_max : 12,
            agi_max : 12,
            inj_decay_rate : 5,
            inj : 0,
            
            bound_neck : true,
            // bound_neck : false,
            // bound_arm : true,
            bound_arm : false,
            bound_hang : true,
            // bound_hang : false,
            bound_wrist : true,
            // bound_wrist : false,
            // bound_joint : true,
            bound_joint : false,
            // bound_thigh : true,
            bound_thigh : false,
            // bound_calve : true,
            bound_calve : false,
            bound_ankle : true,
            // bound_ankle : false,
            bound_long: true,
            // bound_long: false,
            
            fall : false,
            action : true,
            name : "A".to_string(),
            
        }
    }
}