use super::Unit;

fn remove0(num : i32) -> String {
    if num > 0 {
        format!("{:^3}", num)
    }else{
        "   ".to_string()
    }
    
}

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

    pub fn txt_state(&self) -> String {
        let fall = if self.fall {"F"} else {" "};
        let hold = if self.hold {"H"} else {" "};
        format!("{fall}{hold}")
    }

    pub fn txt_attr(&self) -> String {
        let can_stand = if self.can_stand() {"ok "} else {"no "};
        
        let mut txt = String::new();
        // Offense
        txt += &remove0(self.acc_melee_hand());
        txt += &remove0(self.push());
        txt += &remove0(self.hold());
        txt += &remove0(self.tie_power());

        // Denense
        txt += &remove0(self.evd());
        txt += &remove0(self.anti_push());
        txt += &remove0(self.anti_hold());
        txt += &remove0(self.anti_tie_upper());
        txt += &remove0(self.anti_tie_lower());
        
        // Self
        txt += &remove0(self.spd());
        txt += can_stand;
        txt += &remove0(self.hand_dex());
        txt += &remove0(self.hand_agi());
        txt += &remove0(self.unbound_force_upper());
        txt += &remove0(self.unbound_force_lower());
        
        // Basic
        txt += &remove0(self.str()); 
        txt += &remove0(self.dex()); 
        txt += &remove0(self.agi()); 
        txt += &remove0(self.inj);

        txt
    }

    pub fn title_1() -> &'static str {
        "┌─Offense──┐┌───Defense───┐┌─────Self───────┐┌──Basic───┐"
    }

    pub fn title_2() -> &'static str {
        "AccPshHldTieEvdPshHld┌Tie─┐Spd ↑ ┌Hand┐┌Unbd┐StrDexAgiInj"
    }

}