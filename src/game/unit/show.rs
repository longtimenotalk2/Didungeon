use super::Unit;

impl Unit {
    pub fn txt_bound(&self) -> String {
        let upper_type = if self.bound_hang && self.bound_long {
            "="
        } else if !(self.bound_hang || self.bound_long) {
            " "
        } else {
            "-"
        };
        let lower_type = if self.bound_joint && self.bound_long  {
            "="
        } else if !(self.bound_joint || self.bound_long) {
            " "
        } else {
            "-"
        };
        let neck = if self.bound_neck {"@"} else {" "};
        let arm = if self.bound_arm  {"O"} else {upper_type};
        let hang = upper_type;
        let wrist = if self.bound_wrist {"@"} else {" "};
        let joint = lower_type;
        let thigh = if self.bound_thigh  {"0"} else {lower_type} ;
        let calve = if self.bound_calve  {"O"} else {lower_type} ;
        let ankle = if self.bound_ankle {"@"} else {" "};
        format!("[{neck}{arm}{hang}{wrist}{joint}{thigh}{calve}{ankle}]")
    }
}