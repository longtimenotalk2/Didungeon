use serde::{Serialize, Deserialize};

pub enum BoundPart {
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

    pub fn is_bound(&self, part : &BoundPart) -> bool {
        match part {
            BoundPart::Neck => self.bound_neck,
            BoundPart::Arm => self.bound_arm,
            BoundPart::Hang => self.bound_hang,
            BoundPart::Wrist => self.bound_wrist,
            BoundPart::Joint => self.bound_joint,
            BoundPart::Thigh => self.bound_thigh,
            BoundPart::Calve => self.bound_calve,
            BoundPart::Ankle => self.bound_ankle,
            BoundPart::Long => self.bound_long,
        }
    }

    pub fn free_upper(&self) -> bool {
        !(self.is_bound(&BoundPart::Neck) || self.is_bound(&BoundPart::Wrist))
    }

    pub fn free_lower(&self) -> bool {
        !(self.is_bound(&BoundPart::Thigh) || self.is_bound(&BoundPart::Calve) || self.is_bound(&BoundPart::Ankle))
    }

    pub fn is_bound_bow(&self) -> bool {
        self.bound_joint || self.bound_long
    }

    pub fn can_tie_list(&self) -> Vec<BoundPart> {
        let mut list = vec!();
        if !self.bound_neck {list.push(BoundPart::Neck);}
        if !self.bound_arm && self.bound_neck {list.push(BoundPart::Arm);}
        if !self.bound_hang && !self.bound_joint && self.bound_neck && self.bound_wrist {list.push(BoundPart::Hang);}
        if !self.bound_wrist {list.push(BoundPart::Wrist);}
        if !self.bound_joint && !self.bound_hang && self.bound_wrist && self.bound_ankle {list.push(BoundPart::Joint);}
        if !self.bound_thigh {list.push(BoundPart::Thigh);}
        if !self.bound_calve {list.push(BoundPart::Calve);}
        if !self.bound_ankle {list.push(BoundPart::Ankle);}
        if !self.bound_long && self.bound_neck && self.bound_ankle {list.push(BoundPart::Long);}

        list
    }

    pub fn can_untie_list(&self) -> Vec<BoundPart> {
        let mut list = vec!();

        if self.bound_neck {list.push(BoundPart::Neck);}
        if self.bound_arm {list.push(BoundPart::Arm);}
        if self.bound_wrist {list.push(BoundPart::Wrist);}
        if self.bound_thigh {list.push(BoundPart::Thigh);}
        if self.bound_calve {list.push(BoundPart::Calve);}
        if self.bound_ankle {list.push(BoundPart::Ankle);}
        
        list
    }

    pub fn next_force_upper(&self) -> Option<BoundPart> {
        if self.bound_wrist {
            Some(BoundPart::Wrist)
        }else if self.bound_arm {
            Some(BoundPart::Arm)
        }else {
            None
        }
    }

    pub fn next_force_lower(&self) -> Option<BoundPart> {
        if self.bound_ankle {
            Some(BoundPart::Ankle)
        }else if self.bound_calve {
            Some(BoundPart::Calve)
        }else{
            None
        }
    }
}