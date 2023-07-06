mod new;
mod state;
mod show;
// mod test;

#[derive(Clone)]
pub enum Arrow {
    Up,
    Down,
}

impl Arrow {
    pub fn anti(&self) -> Self {
        match self {
            Arrow::Up => Arrow::Down,
            Arrow::Down => Arrow::Up,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Arrow::Up => -1,
            Arrow::Down => 1,
        }
    }
}

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

    pub fn is_upper(&self) -> bool {
        match self {
            Bound::Neck => true,
            Bound::Arm => true,
            Bound::Hang => true,
            Bound::Wrist => true,
            Bound::Joint => false,
            Bound::Thigh => false,
            Bound::Calve => false,
            Bound::Ankle => false,
            Bound::Long => false,
        }
    }
}

pub struct Unit {
    str_max : i32,
    dex_max : i32,
    agi_max : i32,
    inj_coefficient : i32,
    restore_rate : i32,
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
    pub stun : bool,
    
    pub action : bool,

    pub ally : bool,
}

impl Unit {
    pub fn end_turn(&mut self) {
        self.action = true;
        self.auto_restore();
        self.stun = false;
    }

    pub fn restore_amount(&self) -> i32 {
        self.inj * self.restore_rate / 100
    }

    pub fn auto_restore(&mut self) {
        self.inj -= self.restore_amount();
    }

    pub fn be_stun(&mut self) {
        self.stun = true;
        self.action = false;
        self.fall = true;
    }
}