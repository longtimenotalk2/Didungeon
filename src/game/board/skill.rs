use super::Board;
use super::super::unit::Bound;

fn to_hit(h : i32) -> i32 {
    h.max(0).min(100)
}

fn get_is_hit(hit : i32, hit_dice : i32) -> bool {
    return hit >= hit_dice
}

fn txt_hit(target : &str, hit : i32, hit_dice : i32, is_hit : bool, success : &str) -> String {
    format!("  {target} : {hit} -> d100 = {hit_dice} -> {}\n", if is_hit {success} else {"miss"})
}

enum UnboundType {
    ForceUpper,
    ForceLower,
    Hand,
}

impl Board {
    fn unbound_helper(&mut self, ia : i32, bd : &Bound, ubd_type : &UnboundType, txt : &mut String) {
        let a = self.index(ia);
        let hit = to_hit(match ubd_type {
            UnboundType::ForceUpper => a.unbound_force_upper() * 5,
            UnboundType::ForceLower => a.unbound_force_lower() * 5,
            UnboundType::Hand => a.unbound_hand_dex() * 5,
        });
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

    pub fn can_unbound(&self, ia : i32) -> bool {
        let a = self.index(ia);
        a.next_force_upper().is_some() || a.next_force_lower().is_some() || a.next_hand().is_some()
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
            let hit = (agi - times * 5) * 20;
            let hit_dice = self.dice.d(100);
            let is_hit = hit >= hit_dice;

            let new_times = if is_hit {times + 1}else{times};
            
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

    pub fn can_tie(&self, ia : i32, ib : i32) -> bool {
        let a = self.index(ia);
        let b = self.index(ib);

        !a.bound_wrist && b.next_can_tie_choices().len() > 0
    }

    pub fn choice_tie(&self, ia : i32, ib : i32) -> Option<(Bound, bool, i32)> {
        let a = self.index(ia);
        let b = self.index(ib);
        let choices =b.next_can_tie_choices();
        let mut choice : Option<(Bound, bool, i32)> = None;
        let acc = a.tie_power();
        for ch in choices {
            let (bd, is_tie) = ch;
            let evd = if bd.is_upper() {b.anti_tie_upper()} else {b.anti_tie_lower()};
            let hit = to_hit(acc - evd) * 5;
            match choice {
                Some((_, _, hit_)) => {
                    if hit > hit_ {
                        choice = Some((bd, is_tie, hit));
                    }
                },
                None => choice = Some((bd, is_tie, hit)),
            }
        }
        choice
    }

    pub fn tie_with_choice(&mut self, ia : i32, ib : i32) {
        let a = self.index(ia);
        let agi = a.tie_spd();
        let times = agi / 5;
        let hit = (agi - times * 5) * 20;
        let hit_dice = self.dice.d(100);
        let is_hit = hit >= hit_dice;
        let new_times = if is_hit {times + 1}else{times};

        println!("<tie x {times} + {hit}% -> d100 = {hit_dice} -> tie x {} >", new_times);
        
        for _ in 0..times {
            if  let Some((bd, to_tie, hit)) = self.choice_tie(ia, ib) {
                let hit_dice = self.dice.d(100);
                let is_hit = get_is_hit(hit, hit_dice);
                if is_hit {
                    let b = self.index_mut(ib);
                    match bd {
                        Bound::Neck => b.bound_neck = to_tie,
                        Bound::Arm => b.bound_arm = to_tie,
                        Bound::Hang => b.bound_hang = to_tie,
                        Bound::Wrist => b.bound_wrist = to_tie,
                        Bound::Joint => b.bound_joint = to_tie,
                        Bound::Thigh => b.bound_thigh = to_tie,
                        Bound::Calve => b.bound_calve = to_tie,
                        Bound::Ankle => b.bound_ankle = to_tie,
                        Bound::Long => b.bound_long = to_tie,
                    }
                }
                let to_tie_txt = if to_tie {"tie"} else {"untie"};
                let target = format!("{to_tie_txt} {}", bd.txt());
                print!("{}", txt_hit(&target, hit, hit_dice, is_hit, &self.index(ib).txt_bound()));
                if is_hit {
                    let b = self.index(ib);
                    println!("                 {}", b.txt_attr());
                }
            }
        }
    }
}