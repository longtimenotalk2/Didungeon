use super::Board;
use super::super::unit::Bound;

// fn to_hit(h : i32) -> i32 {
//     h.max(0).min(100)
// }

// fn to_dmg(d : i32) -> i32 {
//     d.max(1)
// }

// fn txt_skill(skill : &str, name1 : &str, name2 : Option<&str>) -> String {
//     match name2 {
//         Some(name) => format!("<{skill}> {name1} => {name}\n"),
//         None => format!("<{skill}> {name1}\n"),
//     }
// }

fn txt_hit(txt : &str, hit : i32, hit_dice : i32, is_hit : bool, success : &str) -> String {
    format!("  {txt} : {hit} -> d100 = {hit_dice} -> {}\n", if is_hit {success} else {"miss"})
}

enum UnboundType {
    ForceUpper,
    ForceLower,
    Hand,
}

impl Board {
    fn unbound_helper(&mut self, ia : i32, bd : &Bound, ubd_type : &UnboundType, txt : &mut String) {
        let a = self.index(ia);
        let hit = match ubd_type {
            UnboundType::ForceUpper => a.unbound_force_upper() * 5,
            UnboundType::ForceLower => a.unbound_force_lower() * 5,
            UnboundType::Hand => a.unbound_hand_dex() * 5,
        };
        let hit_dice = self.dice.d(100);
        let is_hit = hit >= hit_dice;
        if is_hit {
            let a = self.index_mut(ia);
            match bd {
                Bound::Neck => {
                    a.bound_neck = false;
                    a.bound_hang = false;
                    a.bound_long = false;
                },
                Bound::Arm => a.bound_arm = false,
                Bound::Hang => a.bound_hang = false,
                Bound::Wrist => {
                    a.bound_wrist = false;
                    a.bound_hang = false;
                    a.bound_joint = false;
                },
                Bound::Joint => a.bound_joint = false,
                Bound::Thigh => a.bound_thigh = false,
                Bound::Calve => a.bound_calve = false,
                Bound::Ankle =>  {
                    a.bound_ankle = false;
                    a.bound_joint = false;
                    a.bound_long = false;
                },
                Bound::Long => a.bound_long = false,
            }
        }
        let a = self.index(ia);
        *txt += &txt_hit(bd.txt(), hit, hit_dice, is_hit, &a.txt_bound());
        if is_hit {
            *txt += &format!("                 {}\n", a.txt_attr())
        }
    }
    
    pub fn unbound(&mut self, ia : i32) {
        
        let mut txt = String::new();

        if let Some(bd) = self.index(ia).next_force_upper() {
            txt += &format!("<unbound upper with force>\n");
            self.unbound_helper(ia, &bd, &UnboundType::ForceUpper, &mut txt);
        }

        if let Some(bd) = self.index(ia).next_force_lower() {
            txt += &format!("<unbound lower with force>\n");
            self.unbound_helper(ia, &bd, &UnboundType::ForceLower, &mut txt);
        }

        if let Some(_) = self.index(ia).next_hand() {
            let a = self.index(ia);

            let agi = a.unbound_hand_agi();
            let times = agi / 5;
            println!("agi={agi}, times={times}");
            let hit = (agi - times * 5) * 20;
            let hit_dice = self.dice.d(100);
            let is_hit = hit >= hit_dice;

            let new_times = if is_hit {times + 1}else{times};
            
            println!("agi={agi}, times={times}");
            txt += &format!("<unbound with hand x {times} + {hit}% -> d100 = {hit_dice} -> hand x {} >\n", new_times);

            
    
            for _ in 0..new_times {
                let a = self.index(ia);
                if let Some(bd) = a.next_hand() {
                    self.unbound_helper(ia, &bd, &UnboundType::Hand, &mut txt);
                }
            }
        }
        print!("{}", txt);

        
    }
}