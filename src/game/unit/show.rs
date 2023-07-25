
use colorful::{Color, Colorful};

use super::Unit;
use super::bound::BoundPart;

const IDENTY: usize = 29;

fn remove0_color(num : i32, color : Option<Color>) -> String {
    let a = if num != 0 {
        format!("{:^3}", num)
    }else{
        "   ".to_string()
    };
    if let Some(color) = color {
        a.color(color).to_string()
    }else{
        a
    }
}

fn remove0(num : i32) -> String {
    let a = if num != 0 {
        format!("{:^3}", num)
    }else{
        "   ".to_string()
    };
    a
}

fn spaces(intend : usize) -> String {
    " ".repeat(intend)
}

impl Unit {
    pub fn show(&self, is_act : bool) {
        Self::is_act(is_act);
        print!(" ");
        print!("{}", self.action());
        print!("{}", self.identity_for_pure_chinese());
        print!(" ");
        print!("{} ", self.aim());
        print!("{} ", self.state());
        print!("{}", self.bound());
        print!(" ");
        print!("{}", self.attr());
        print!("\n");
    }

    pub fn identity_wo_attr(&self) -> String {
        let s = format!("{}{} {} {}{}", self.action(), self.identity_for_pure_chinese(), self.aim(), self.state(), self.bound());
        s
    }

    pub fn identity_basic_attr(&self, is_actor : bool, is_next : bool) -> String {
        let mut s = String::new();
        // Spd
        if is_actor {
            s += &remove0(self.spd());
        } else {
            if is_next {
                s += &remove0_color(self.spd(), Some(Color::Yellow))
            }else{
                if self.action {
                    match self.wait {
                        true => s += &remove0_color(self.spd(), Some(Color::Green)),
                        false => s += &remove0_color(self.spd(), Some(Color::Red)),
                    }
                }else{
                    s += &remove0_color(self.spd(), Some(Color::Grey0));
                }
            }
        }
        // Mov
        if is_actor {
            s += &remove0_color(self.move_range(), None);
        }else{
            s += &remove0_color(self.move_range(), Some(Color::Grey0));
        }
        // Hurt
        s += &remove0_color(self.inj, Some(Color::Red));
        // Basic 
        if is_actor {
            s += &remove0_color(self.str_adj(), None);
            s += &remove0_color(self.dex_adj(), None); 
            s += &remove0_color(self.agi_adj(), None); 
        }else{
            s += &remove0_color(self.str_adj(), Some(Color::Grey0));
            s += &remove0_color(self.dex_adj(), Some(Color::Grey0)); 
            s += &remove0_color(self.agi_adj(), Some(Color::Grey0)); 
        }
        
        s
    }

    pub fn show_title1() {
        println!("{}{}", spaces(IDENTY), Self::title_1());
    }

    pub fn show_title2() {
        println!("{}{}", spaces(IDENTY), Self::title_2());
    }

    pub fn identity(&self) -> String {
        self.name.clone()
    }

    pub fn show_identity(&self) {
        print!("{}", self.identity())
    }

    fn is_act(is_act : bool) {
        match is_act {
            true => print!(">"),
            false => print!(" "),
        }
    }

    fn identity_for_pure_chinese(&self) -> &str {
        &self.name_fix_length
    }

    fn action(&self) -> &str {
        if self.action {
            "|"
        }else{
            " "
        }
    }

    fn aim(&self) -> String {
        self.dir.notice().to_string()
    }

    fn state(&self) -> String {
        // 发布版
        let a = if self.is_defeated() {
            "🏳️ "
        }else if self.is_stun() {
            "💫 "
        }else if self.shock {
            "惊 "
        }else if self.is_sleep() {
            "💤 "
        }else if self.is_fall() {
            "🧎 "
        }else if let Some(_) = self.catch_left {
            "👆 "
        }else if let Some(_) = self.catch_right {
            "👇 "
        }else {
            "   "
        };
        a.to_string()

        // 自用
        // let a = if self.is_defeated() {
        //     "🏳️ "
        // }else if self.is_stun() {
        //     "💫 "
        // }else if self.shock {
        //     "惊 "
        // }else if self.is_sleep() {
        //     "💤 "
        // }else if self.is_fall() {
        //     "🧎"
        // }else if let Some(_) = self.catch_left {
        //     "👆 "
        // }else if let Some(_) = self.catch_right {
        //     "👇 "
        // }else {
        //     "   "
        // };
        // format!("{}{}", a, "".to_string().color(Color::White).to_string())
    }

    fn bound(&self) -> String {
        self.bound.identity(None, true)
    }

    pub fn bound_identity(&self, bound_case : Option<(&BoundPart, bool)>, show_loose : bool) -> String {
        self.bound.identity(bound_case, show_loose)
    }

    pub fn identity_tightness(&self, part : &BoundPart) -> String {
        self.bound.identity_tightness(part)
    }

    pub fn attr(&self) -> String{
        
        let mut txt = String::new();

        // Spd
        txt += &remove0(self.spd());
        
        // Hurt
        txt += &remove0_color(self.inj, Some(Color::Red));

        // Basic 
        txt += &remove0_color(self.str_adj(), None);
        txt += &remove0_color(self.dex_adj(), None); 
        txt += &remove0_color(self.agi_adj(), None); 

        // Offense
        txt += &remove0(self.acc_melee_hand());
        txt += &remove0(self.atk_melee_hand());
        txt += &remove0(self.hold_force());
        txt += &remove0(self.tie_power());

        // Denense
        txt += &remove0(self.evd());
        txt += &remove0(self.def_gym());
        txt += &remove0(self.struggle_force());
        txt += &remove0(self.anti_tie_upper());
        txt += &remove0(self.anti_tie_lower());

        // Hand
        txt += &remove0(self.hand_str());
        txt += &remove0(self.hand_dex());

        // Self
        txt += if self.can_stand() {"ok "} else {"no "};
        txt += &remove0(self.move_range());
        txt += &remove0(self.unbound_force_upper());
        txt += &remove0(self.unbound_force_lower());
        

        txt
    }


    // pub fn attr(&self) {
    //     let can_stand = if self.can_stand() {"ok "} else {"no "};
        
    //     let mut txt = String::new();
    //     // Offense
    //     txt += &remove0(self.acc_melee_hand());
    //     txt += &remove0(self.atk_melee_hand());
    //     txt += &remove0(self.hold_force());
    //     txt += &remove0(self.tie_power());

    //     // Denense
    //     txt += &remove0(self.evd());
    //     txt += &remove0(self.def_gym());
    //     txt += &remove0(self.struggle_force());
    //     txt += &remove0(self.anti_tie_upper());
    //     txt += &remove0(self.anti_tie_lower());

    //     // Hand
    //     txt += &remove0(self.hand_str());
    //     txt += &remove0(self.hand_dex());

    //     // Self
    //     txt += &remove0(self.spd());
    //     txt += can_stand;
    //     txt += &remove0(self.move_range());
    //     txt += &remove0(self.unbound_force_upper());
    //     txt += &remove0(self.unbound_force_lower());
        
    //     // Basic
    //     txt += &remove0(self.str_adj()); 
    //     txt += &remove0(self.dex_adj()); 
    //     txt += &remove0(self.agi_adj()); 
    //     txt += &remove0(self.inj);

    //     print!("{}", txt);
    // }

    fn title_1() -> &'static str {
        "┌────基础─────┐┌──进攻端──┐┌───防御端────┐┌手部┐┌───自身───┐"
    }

    fn title_2() -> &'static str {
        "速 伤 力 技 敏 命 攻 压 捆 回 防 挣 ┌反捆┐力 技 起 移 ┌脱缚┐"
        // "命 攻 压 捆 回 防 挣 ┌反捆┐力 技 速 起 移 ┌脱缚┐力 技 敏 伤 "
    }
}