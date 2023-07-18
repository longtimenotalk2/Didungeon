use colorful::{Color, Colorful};
use serde::{Serialize, Deserialize};
use std::fmt::Write;

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn name(&self) -> &'static str {
        match self {
            BoundPart::Neck => "五花",
            BoundPart::Arm => "大臂",
            BoundPart::Hang => "手腕<-->后颈",
            BoundPart::Wrist => "手腕",
            BoundPart::Joint => "脚腕<-->手腕",
            BoundPart::Thigh => "大腿",
            BoundPart::Calve => "小腿",
            BoundPart::Ankle => "脚腕",
            BoundPart::Long => "脚腕<-->后颈",
        }
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
            BoundPart::Neck => self.bound_neck = 100,
            BoundPart::Arm => self.bound_arm = 100,
            BoundPart::Hang => self.bound_hang = 100,
            BoundPart::Wrist => self.bound_wrist = 100,
            BoundPart::Joint => self.bound_joint = 100,
            BoundPart::Thigh => self.bound_thigh = 100,
            BoundPart::Calve => self.bound_calve = 100,
            BoundPart::Ankle => self.bound_ankle = 100,
            BoundPart::Long => self.bound_long = 100,
        }
    }

    pub fn untie(&mut self, part : &BoundPart) {
        match part {
            BoundPart::Neck => self.bound_neck = 0,
            BoundPart::Arm => self.bound_arm = 0,
            BoundPart::Hang => self.bound_hang = 0,
            BoundPart::Wrist => self.bound_wrist = 0,
            BoundPart::Joint => self.bound_joint = 0,
            BoundPart::Thigh => self.bound_thigh = 0,
            BoundPart::Calve => self.bound_calve = 0,
            BoundPart::Ankle => self.bound_ankle = 0,
            BoundPart::Long => self.bound_long = 0,
        }
    }

    pub fn tightness_change_to(&mut self, part : &BoundPart, num : i32) {
        match part {
            BoundPart::Neck => self.bound_neck = num,
            BoundPart::Arm => self.bound_arm = num,
            BoundPart::Hang => self.bound_hang = num,
            BoundPart::Wrist => self.bound_wrist = num,
            BoundPart::Joint => self.bound_joint = num,
            BoundPart::Thigh => self.bound_thigh = num,
            BoundPart::Calve => self.bound_calve = num,
            BoundPart::Ankle => self.bound_ankle = num,
            BoundPart::Long => self.bound_long = num,
        }
    }

    pub fn is_bound(&self, part : &BoundPart) -> bool {
        match part {
            BoundPart::Neck => self.bound_neck > 0,
            BoundPart::Arm => self.bound_arm> 0,
            BoundPart::Hang => self.bound_hang> 0,
            BoundPart::Wrist => self.bound_wrist> 0,
            BoundPart::Joint => self.bound_joint> 0,
            BoundPart::Thigh => self.bound_thigh> 0,
            BoundPart::Calve => self.bound_calve> 0,
            BoundPart::Ankle => self.bound_ankle> 0,
            BoundPart::Long => self.bound_long> 0,
        }
    }

    pub fn get_tightness(&self, part : &BoundPart) -> i32 {
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

    pub fn is_bound_bow(&self) -> bool {
        self.is_bound_joint() || self.is_bound_long()
    }

    pub fn can_tie_list(&self) -> Vec<BoundPart> {
        let mut list = vec!();
        if !self.is_bound_neck() {list.push(BoundPart::Neck);}
        if !self.is_bound_arm() && self.is_bound_neck() {list.push(BoundPart::Arm);}
        if !self.is_bound_hang() && !self.is_bound_joint() && self.is_bound_neck() && self.is_bound_wrist() {list.push(BoundPart::Hang);}
        if !self.is_bound_wrist() {list.push(BoundPart::Wrist);}
        if !self.is_bound_joint() && !self.is_bound_hang() && self.is_bound_wrist() && self.is_bound_ankle() {list.push(BoundPart::Joint);}
        if !self.is_bound_thigh() {list.push(BoundPart::Thigh);}
        if !self.is_bound_calve() {list.push(BoundPart::Calve);}
        if !self.is_bound_ankle() {list.push(BoundPart::Ankle);}
        if !self.is_bound_long() && self.is_bound_neck() && self.is_bound_ankle() {list.push(BoundPart::Long);}

        list
    }

    pub fn can_untie_list(&self) -> Vec<BoundPart> {
        let mut list = vec!();

        if self.is_bound_neck() && !self.is_bound_hang() && !self.is_bound_long() && !self.is_bound_arm() {list.push(BoundPart::Neck);}
        if self.is_bound_arm() {list.push(BoundPart::Arm);}
        if self.is_bound_hang() {list.push(BoundPart::Hang);}
        if self.is_bound_wrist() && !self.is_bound_hang() && !self.is_bound_joint() {list.push(BoundPart::Wrist);}
        if self.is_bound_joint() {list.push(BoundPart::Joint);}
        if self.is_bound_thigh() {list.push(BoundPart::Thigh);}
        if self.is_bound_calve() {list.push(BoundPart::Calve);}
        if self.is_bound_ankle() && !self.is_bound_long() && !self.is_bound_joint() {list.push(BoundPart::Ankle);}
        if self.is_bound_long() {list.push(BoundPart::Long);}
        
        list
    }

    pub fn next_force_upper(&self) -> Option<BoundPart> {
        if self.is_bound_wrist() {
            Some(BoundPart::Wrist)
        }else if self.is_bound_arm() {
            Some(BoundPart::Arm)
        }else {
            None
        }
    }

    pub fn next_force_lower(&self) -> Option<BoundPart> {
        if self.is_bound_ankle() {
            Some(BoundPart::Ankle)
        }else if self.is_bound_calve() {
            Some(BoundPart::Calve)
        }else{
            None
        }
    }

    pub fn show(&self) {
        let upper_type = if self.is_bound_hang() && self.is_bound_long() {
            "="
        } else if !(self.is_bound_hang() || self.is_bound_long()) {
            " "
        } else {
            "-"
        };
        let lower_type = if self.is_bound_joint() && self.is_bound_long()  {
            "="
        } else if !(self.is_bound_joint() || self.is_bound_long()) {
            " "
        } else {
            "-"
        };
        let neck = if self.is_bound_neck() {"@"} else {" "};
        let arm = if self.is_bound_arm()  {"O"} else {upper_type};
        let hang = upper_type;
        let wrist = if self.is_bound_wrist() {"@"} else {" "};
        let joint = lower_type;
        let thigh = if self.is_bound_thigh()  {"0"} else {lower_type} ;
        let calve = if self.is_bound_calve()  {"O"} else {lower_type} ;
        let ankle = if self.is_bound_ankle() {"@"} else {" "};
        print!("[{neck}{arm}{hang}{wrist}{joint}{thigh}{calve}{ankle}]")
    }

    pub fn identity_tightness(&self, bound : &BoundPart) -> String {
        let tightness = self.get_tightness(bound);
        if 0 < tightness && tightness < 100 {
            let a = format!("({}%)", tightness);
            let mut s = String::new();
            write!(&mut s, "{}", a.color(Color::Red)).unwrap();
            s
        }else{
            String::new()
        }
    }

    pub fn identity_change(&self, part : &BoundPart, is_tie : bool) -> String {
        let color = match is_tie {
            true => Color::Green,
            false => Color::Red,
        };
        let default = Color::White;
        let mut new = self.clone();
        if is_tie {
            new.tie(part);
        }

        let upper_type = if new.is_bound_hang() && new.is_bound_long() {
            "="
        } else if !(new.is_bound_hang() || new.is_bound_long()) {
            " "
        } else {
            "-"
        };
        let lower_type = if new.is_bound_joint() && new.is_bound_long()  {
            "="
        } else if !(new.is_bound_joint() || new.is_bound_long()) {
            " "
        } else {
            "-"
        };
        let neck = if new.is_bound_neck() {
            if BoundPart::Neck == *part {
                "@".color(color)
            }else {
                "@".color(default)
            }
        } else {
            " ".to_string().color(default)
        };
        let arm = if new.is_bound_arm()  {
            if BoundPart::Arm == *part {
                "O".color(color)
            }else {
                "O".color(default)
            }
        } else {
            if BoundPart::Hang == *part || BoundPart::Long == *part {
                upper_type.color(color)
            }else {
                upper_type.color(default)
            }
        };
        let hang = if BoundPart::Hang == *part || BoundPart::Long == *part {
            upper_type.color(color)
        }else {
            upper_type.color(default)
        };
        let wrist = if new.is_bound_wrist() {
            if BoundPart::Wrist == *part {
                "@".color(color)
            }else {
                "@".color(default)
            }
        } else {
            " ".to_string().color(default)
        };
        let joint = if BoundPart::Joint == *part || BoundPart::Long == *part {
            lower_type.color(color)
        }else {
            lower_type.color(default)
        };

        let thigh = if new.is_bound_thigh()  {
            if BoundPart::Thigh == *part {
                "0".color(color)
            }else {
                "0".color(default)
            }
        } else {
            if BoundPart::Joint == *part || BoundPart::Long == *part {
                lower_type.color(color)
            }else {
                lower_type.color(default)
            }
        };

        let calve = if new.is_bound_calve()  {
            if BoundPart::Calve == *part {
                "O".color(color)
            }else {
                "O".color(default)
            }
        } else {
            if BoundPart::Joint == *part || BoundPart::Long == *part {
                lower_type.color(color)
            }else {
                lower_type.color(default)
            }
        };
        
        let ankle = if new.is_bound_ankle() {
            if BoundPart::Ankle == *part {
                "@".color(color)
            }else {
                "@".color(default)
            }
        } else {
            " ".to_string().color(default)
        };

        let mut s = String::new();
        write!(s, "[{neck}{arm}{hang}{wrist}{joint}{thigh}{calve}{ankle}]").unwrap();
        s
    }

    pub fn ai_tie_choice(&self) -> Option<(BoundPart, bool)> {
        if !self.is_bound_wrist() {
            Some((BoundPart::Wrist, true))
        }else if !self.is_bound_ankle() {
            Some((BoundPart::Ankle, true))
        }else if !self.is_bound_hang() && self.is_bound_neck() && !self.is_bound_joint() {
            Some((BoundPart::Hang, true))
        }else if !self.is_bound_long() && self.is_bound_neck() {
            Some((BoundPart::Long, true))
        }else if !self.is_bound_joint() && !self.is_bound_hang() {
            Some((BoundPart::Joint, true))
        }else if !self.is_bound_neck() {
            Some((BoundPart::Neck, true))
        }else if !self.is_bound_arm() {
            Some((BoundPart::Arm, true))
        }else if !self.is_bound_calve() {
            Some((BoundPart::Calve, true))
        }else if !self.is_bound_thigh() {
            Some((BoundPart::Thigh, true))
        }else if !self.is_bound_hang() {
            Some((BoundPart::Joint, false))
        }else{
            None
        }
    }
}