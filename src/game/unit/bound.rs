mod show;


use serde::{Serialize, Deserialize};

use self::BoundPart::Neck as Neck;
use self::BoundPart::Arm as Arm;
use self::BoundPart::Hang as Hang;
use self::BoundPart::Wrist as Wrist;
use self::BoundPart::Joint as Joint;
use self::BoundPart::Thigh as Thigh;
use self::BoundPart::Calve as Calve;
use self::BoundPart::Ankle as Ankle;
use self::BoundPart::Long as Long;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

impl BoundPart {
    pub fn all() -> Vec<Self> {
        vec![
            BoundPart::Neck,
            BoundPart::Arm,
            BoundPart::Hang,
            BoundPart::Wrist,
            BoundPart::Joint,
            BoundPart::Thigh,
            BoundPart::Calve,
            BoundPart::Ankle,
            BoundPart::Long,
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoundState {
    bound_neck : i32,
    bound_arm : i32,
    bound_hang : i32,
    bound_wrist : i32,
    bound_joint : i32,
    bound_thigh : i32,
    bound_calve : i32,
    bound_ankle : i32,
    bound_long : i32,
}

impl BoundState {
    pub fn new() -> Self {
        Self {
            bound_neck : 0,
            bound_arm : 0,
            bound_hang : 0,
            bound_wrist : 0,
            bound_joint : 0,
            bound_thigh : 0,
            bound_calve : 0,
            bound_ankle : 0,
            bound_long : 0,
        }
    }

    pub fn tie(&mut self, part : &BoundPart) {
        match part {
            Neck => self.bound_neck = 100,
            Arm => self.bound_arm = 100,
            Hang => self.bound_hang = 100,
            Wrist => self.bound_wrist = 100,
            Joint => self.bound_joint = 100,
            Thigh => self.bound_thigh = 100,
            Calve => self.bound_calve = 100,
            Ankle => self.bound_ankle = 100,
            Long => self.bound_long = 100,
        }
    }

    pub fn tie_full(&mut self) {
        self.bound_neck = 100;
        self.bound_arm = 100;
        self.bound_hang = 100;
        self.bound_wrist = 100;
        self.bound_thigh = 100;
        self.bound_calve = 100;
        self.bound_ankle = 100;
        self.bound_long = 100;
    }

    pub fn untie(&mut self, part : &BoundPart) {
        match part {
            Neck => self.bound_neck = 0,
            Arm => self.bound_arm = 0,
            Hang => self.bound_hang = 0,
            Wrist => self.bound_wrist = 0,
            Joint => self.bound_joint = 0,
            Thigh => self.bound_thigh = 0,
            Calve => self.bound_calve = 0,
            Ankle => self.bound_ankle = 0,
            Long => self.bound_long = 0,
        }
    }

    pub fn tightness_change_to(&mut self, part : &BoundPart, num : i32) {
        match part {
            Neck => self.bound_neck = num,
            Arm => self.bound_arm = num,
            Hang => self.bound_hang = num,
            Wrist => self.bound_wrist = num,
            Joint => self.bound_joint = num,
            Thigh => self.bound_thigh = num,
            Calve => self.bound_calve = num,
            Ankle => self.bound_ankle = num,
            Long => self.bound_long = num,
        }
    }

    pub fn has_bound(&self) -> bool {
        let mut r = false;
        for b in BoundPart::all(){
            if self.is_bound(&b) {
                r = true;
            }
        }
        r
    }

    pub fn is_bound(&self, part : &BoundPart) -> bool {
        match part {
            Neck => self.bound_neck > 0,
            Arm => self.bound_arm> 0,
            Hang => self.bound_hang> 0,
            Wrist => self.bound_wrist> 0,
            Joint => self.bound_joint> 0,
            Thigh => self.bound_thigh> 0,
            Calve => self.bound_calve> 0,
            Ankle => self.bound_ankle> 0,
            Long => self.bound_long> 0,
        }
    }

    pub fn get_tightness(&self, part : &BoundPart) -> i32 {
        match part {
            Neck => self.bound_neck,
            Arm => self.bound_arm,
            Hang => self.bound_hang,
            Wrist => self.bound_wrist,
            Joint => self.bound_joint,
            Thigh => self.bound_thigh,
            Calve => self.bound_calve,
            Ankle => self.bound_ankle,
            Long => self.bound_long,
        }
    }

    pub fn free_upper(&self) -> bool {
        !(self.is_bound(&Neck) || self.is_bound(&Wrist))
    }

    pub fn free_lower(&self) -> bool {
        !(self.is_bound(&Thigh) || self.is_bound(&Calve) || self.is_bound(&Ankle))
    }

    fn is_bound_neck(&self) -> bool {
        self.bound_neck > 0
    }
    fn is_bound_arm(&self) -> bool {
        self.bound_arm > 0
    }
    fn is_bound_hang(&self) -> bool {
        self.bound_hang > 0
    }
    fn is_bound_wrist(&self) -> bool {
        self.bound_wrist > 0
    }
    fn is_bound_joint(&self) -> bool {
        self.bound_joint > 0
    }
    fn is_bound_thigh(&self) -> bool {
        self.bound_thigh > 0
    }
    fn is_bound_calve(&self) -> bool {
        self.bound_calve > 0
    }
    fn is_bound_ankle(&self) -> bool {
        self.bound_ankle > 0
    }
    fn is_bound_long(&self) -> bool {
        self.bound_long > 0
    }

    
    fn is_loose_neck(&self) -> bool {
        self.bound_neck < 100 
    }
    fn is_loose_arm(&self) -> bool {
        self.bound_arm < 100
    }
    fn is_loose_hang(&self) -> bool {
        self.bound_hang < 100
    }
    fn is_loose_wrist(&self) -> bool {
        self.bound_wrist < 100
    }
    fn is_loose_joint(&self) -> bool {
        self.bound_joint < 100
    }
    fn is_loose_thigh(&self) -> bool {
        self.bound_thigh < 100
    }
    fn is_loose_calve(&self) -> bool {
        self.bound_calve < 100
    }
    fn is_loose_ankle(&self) -> bool {
        self.bound_ankle < 100
    }
    fn is_loose_long(&self) -> bool {
        self.bound_long < 100
    }

    pub fn is_bound_bow(&self) -> bool {
        self.is_bound_joint() || self.is_bound_long()
    }

    pub fn is_defeated(&self) -> bool {
        self.is_bound_neck() && self.is_bound_arm() && self.is_bound_wrist() && self.is_bound_hang() && self.is_bound_thigh() && self.is_bound_calve() && self.is_bound_ankle() && self.is_bound_long()
    }

    pub fn can_tie_list(&self) -> Vec<BoundPart> {
        let mut list = vec!();
        if !self.is_bound_neck() {list.push(Neck);}
        if !self.is_bound_arm() && self.is_bound_neck() {list.push(Arm);}
        if !self.is_bound_hang() && !self.is_bound_joint() && self.bound_neck == 100 && self.bound_wrist == 100 {list.push(Hang);}
        if !self.is_bound_wrist() {list.push(Wrist);}
        if !self.is_bound_joint() && !self.is_bound_hang() && self.bound_wrist == 100 && self.bound_ankle == 100 {list.push(Joint);}
        if !self.is_bound_thigh() {list.push(Thigh);}
        if !self.is_bound_calve() {list.push(Calve);}
        if !self.is_bound_ankle() {list.push(Ankle);}
        if !self.is_bound_long() && self.bound_neck == 100 && self.bound_ankle == 100 {list.push(Long);}

        list
    }

    pub fn can_untie_list(&self) -> Vec<BoundPart> {
        let mut list = vec!();

        if self.is_bound_neck() && !self.is_bound_hang() && !self.is_bound_long() && !self.is_bound_arm() {list.push(Neck);}
        if self.is_bound_arm() {list.push(Arm);}
        if self.is_bound_hang() {list.push(Hang);}
        if self.is_bound_wrist() && !self.is_bound_hang() && !self.is_bound_joint() {list.push(Wrist);}
        if self.is_bound_joint() {list.push(Joint);}
        if self.is_bound_thigh() {list.push(Thigh);}
        if self.is_bound_calve() {list.push(Calve);}
        if self.is_bound_ankle() && !self.is_bound_long() && !self.is_bound_joint() {list.push(Ankle);}
        if self.is_bound_long() {list.push(Long);}
        
        list
    }

    pub fn next_force_upper(&self) -> Option<BoundPart> {
        if self.is_bound_hang() {
            Some(Hang)
        }else if self.is_bound_joint() {
            Some(Joint)
        }else if self.is_bound_wrist() {
            Some(Wrist)
        }else if self.is_bound_arm() {
            Some(Arm)
        }else {
            None
        }
    }

    pub fn next_force_lower(&self) -> Option<BoundPart> {
        if self.is_bound_long() {
            Some(Long)
        }else if self.is_bound_joint() {
            Some(Joint)
        }else if self.is_bound_ankle() {
            Some(Ankle)
        }else if self.is_bound_calve() {
            Some(Calve)
        }else {
            None
        }
    }

    pub fn ai_tie_choice(&self) -> Option<(BoundPart, bool)> {
        if !self.is_bound_wrist() {
            Some((Wrist, true))
        }else if !self.is_bound_ankle() {
            Some((Ankle, true))
        }else if !self.is_bound_hang() && self.is_bound_neck() && !self.is_bound_joint() {
            Some((Hang, true))
        }else if !self.is_bound_long() && self.is_bound_neck() {
            Some((Long, true))
        }else if !self.is_bound_joint() && !self.is_bound_hang() {
            Some((Joint, true))
        }else if !self.is_bound_neck() {
            Some((Neck, true))
        }else if !self.is_bound_arm() {
            Some((Arm, true))
        }else if !self.is_bound_calve() {
            Some((Calve, true))
        }else if !self.is_bound_thigh() {
            Some((Thigh, true))
        }else if !self.is_bound_hang() {
            Some((Joint, false))
        }else{
            None
        }
    }

    pub fn ai_unbound_choice(&self) -> Option<BoundPart> {
        if self.is_bound_arm() {
            Some(Arm)
        }else if self.is_bound_long() {
            Some(Long)
        }else if self.is_bound_neck() {
            Some(Neck)
        }else if self.is_bound_ankle() {
            Some(Ankle)
        }else if self.is_bound_calve() {
            Some(Calve)
        }else if self.is_bound_thigh() {
            Some(Thigh)
        }else {
            None
        }
            
    }
}