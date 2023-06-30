mod state;
mod show;
mod test;

pub enum Bound {
    Neck,
    Arm,
    Hang,
    Wrist,
    Joint,
    Thigh,
    Calve,
    Ankle, 
    Long,
}

impl Bound {
    pub fn txt(&self) -> &'static str {
        match self {
            Bound::Neck => "neck",
            Bound::Arm => "arm",
            Bound::Hang => "hang",
            Bound::Wrist => "wrist",
            Bound::Joint => "joint",
            Bound::Thigh => "thigh",
            Bound::Calve => "calve",
            Bound::Ankle => "ankle",
            Bound::Long => "long",
            
        }
    }
}

pub struct Unit {
    str_max : i32,
    dex_max : i32,
    agi_max : i32,
    inj_decay_rate : i32,
    pub inj : i32,

    pub bound_neck : bool,
    pub bound_arm : bool,
    pub bound_hang : bool,
    pub bound_wrist : bool,
    pub bound_joint : bool,
    pub bound_thigh : bool,
    pub bound_calve : bool,
    pub bound_ankle : bool,
    pub bound_long : bool,

    pub fall : bool,
    pub hold : bool,
    
    pub action : bool,

    pub name : String,
}