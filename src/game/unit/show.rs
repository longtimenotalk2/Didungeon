
use super::Unit;
use super::bound::BoundPart;
use std::fmt::Write;

use colorful::Color;
use colorful::Colorful;

const IDENTY: usize = 26;

fn remove0(num : i32) -> String {
    if num > 0 {
        format!("{:^3}", num)
    }else{
        "   ".to_string()
    }
}

fn spaces(intend : usize) -> String {
    " ".repeat(intend)
}

impl Unit {
    pub fn show(&self, is_act : bool) {
        Self::is_act(is_act);
        print!(" ");
        self.action();
        self.identity_for_pure_chinese();
        print!(" ");
        self.state();
        print!(" ");
        self.bound();
        print!(" ");
        self.attr();
        print!("\n");
    }

    pub fn show_title1() {
        println!("{}{}", spaces(IDENTY), Self::title_1());
    }

    pub fn show_title2() {
        println!("{}{}", spaces(IDENTY), Self::title_2());
    }

    pub fn identity(&self) -> String {
        let name_color = match self.you {
            true => Color::Green,
            false => match self.ally {
                true => Color::Blue,
                false => Color::Red,
            },
        };
        let mut s = String::new();
        write!(&mut s, "{}", self.name.clone().color(name_color)).unwrap();
        s
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

    fn identity_for_pure_chinese(&self) {
        let len = self.name.len();
        let adder = " ".repeat(8-len/3*2);
        self.show_identity();
        print!("{}", adder);
    }

    fn action(&self) {
        if self.action {
            print!("|");
        }else{
            print!(" ");
        }
    }

    fn state(&self) {
        if self.is_stun() {
            print!("💫")
        }else if self.is_sleep() {
            print!("💤")
        }else if self.is_fall() {
            print!("🧎")
        }else if let Some(_) = self.catch_left {
            print!("👆")
        }else if let Some(_) = self.catch_right {
            print!("👇")
        }else {
            print!("  ")
        }
    }

    fn bound(&self) {
        self.bound.show()
    }

    pub fn bound_identity_change(&self, part : &BoundPart, is_tie : bool) -> String {
        self.bound.identity_change(part, is_tie)
    }

    pub fn attr(&self) {
        let can_stand = if self.can_stand() {"ok "} else {"no "};
        
        let mut txt = String::new();
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
        txt += &remove0(self.spd());
        txt += can_stand;
        txt += &remove0(self.move_range());
        txt += &remove0(self.unbound_force_upper());
        txt += &remove0(self.unbound_force_lower());
        
        // Basic
        txt += &remove0(self.str_adj()); 
        txt += &remove0(self.dex_adj()); 
        txt += &remove0(self.agi_adj()); 
        txt += &remove0(self.inj);

        print!("{}", txt);
    }

    fn title_1() -> &'static str {
        "┌──进攻端──┐┌───防御端────┐┌手部┐┌────自身─────┐┌───基础───┐"
    }

    fn title_2() -> &'static str {
        "命 攻 压 捆 回 防 挣 ┌反捆┐力 技 速 起 移 ┌脱缚┐力 技 敏 伤 "
    }
}