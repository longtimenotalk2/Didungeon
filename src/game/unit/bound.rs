use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BoundState {
    bound_neck : bool,
    bound_arm : bool,
    bound_hang : bool,
    bound_wrist : bool,
    bound_joint : bool,
    bound_thigh : bool,
    bound_calve : bool,
    bound_ankle : bool,
    bound_long : bool,

    tie_process_neck : i32,
    tie_process_arm : i32,
    tie_process_hang : i32,
    tie_process_wrist : i32,
    tie_process_joint : i32,
    tie_process_thigh : i32,
    tie_process_calve : i32,
    tie_process_ankle : i32,
    tie_process_long : i32,

    untie_process_neck : i32,
    untie_process_arm : i32,
    untie_process_hang : i32,
    untie_process_wrist : i32,
    untie_process_joint : i32,
    untie_process_thigh : i32,
    untie_process_calve : i32,
    untie_process_ankle : i32,
    untie_process_long : i32,
}

impl BoundState {
    pub fn new() -> Self {
        Self {
            bound_neck : false,
            bound_arm : false,
            bound_hang : false,
            bound_wrist : false,
            bound_joint : false,
            bound_thigh : false,
            bound_calve : false,
            bound_ankle : false,
            bound_long : false,

            tie_process_neck : 0,
            tie_process_arm : 0,
            tie_process_hang : 0,
            tie_process_wrist : 0,
            tie_process_joint : 0,
            tie_process_thigh : 0,
            tie_process_calve : 0,
            tie_process_ankle : 0,
            tie_process_long : 0,

            untie_process_neck : 0,
            untie_process_arm : 0,
            untie_process_hang : 0,
            untie_process_wrist : 0,
            untie_process_joint : 0,
            untie_process_thigh : 0,
            untie_process_calve : 0,
            untie_process_ankle : 0,
            untie_process_long : 0,
        }
    }
}