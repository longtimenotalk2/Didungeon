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
    Cuter,
}

impl Board {
    fn unbound_helper(&mut self, ia : i32, bd : &Bound, ubd_type : &UnboundType, txt : &mut String) {
        let a = self.index(ia);
        let hit = to_hit(match ubd_type {
            UnboundType::ForceUpper => a.unbound_force_upper() * 5,
            UnboundType::ForceLower => a.unbound_force_lower() * 5,
            UnboundType::Hand => a.unbound_hand_dex() * 5,
            UnboundType::Cuter => a.spd() * 5,
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
            *txt += &format!("{}{}\n", Board::title_front(), a.txt_attr())
        }
    }

    pub fn can_unbound(&self, ia : i32) -> bool {
        let a = self.index(ia);
        a.next_force_upper().is_some() || a.next_force_lower().is_some() || (!a.hold && a.next_hand().is_some())
    }

    pub fn hit_unbound(&self, ia : i32) -> i32 {
        let mut hit_now = 0;
        let a = self.index(ia);
        if let Some(_) = a.next_force_upper() {
            let hit = to_hit(a.unbound_force_upper() * 5);
            hit_now = hit_now.max(hit);
        };
        if let Some(_) = a.next_force_lower() {
            let hit = to_hit(a.unbound_force_lower() * 5);
            hit_now = hit_now.max(hit);
        };
        if !self.index(ia).hold {
            if let Some(_) = a.next_hand() {
                let hit = to_hit(a.unbound_hand_dex() * 5);
                hit_now = hit_now.max(hit);
            }
        }
        if !a.fall && a.bound_hang {
            let hit = to_hit(a.spd() * 5);
            hit_now = hit_now.max(hit);
        }
        
        hit_now
    }
    
    pub fn unbound(&mut self, ia : i32) {
        
        let mut txt = String::new();

        let a = self.index(ia);
        if !a.fall && a.bound_hang {
            txt += &format!("<unbound hang with cuter>\n");
            self.unbound_helper(ia, &Bound::Hang, &UnboundType::Cuter, &mut txt);
        }

        if let Some(bd) = self.index(ia).next_force_upper() {
            txt += &format!("<unbound upper with force>\n");
            self.unbound_helper(ia, &bd, &UnboundType::ForceUpper, &mut txt);
        }

        if let Some(bd) = self.index(ia).next_force_lower() {
            txt += &format!("<unbound lower with force>\n");
            self.unbound_helper(ia, &bd, &UnboundType::ForceLower, &mut txt);
        }

        if !self.index(ia).hold {
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
        }
        print!("{}", txt);
    }

    pub fn can_tie(&self, ia : i32, ib : i32) -> bool {
        let a = self.index(ia);
        let b = self.index(ib);

        !a.bound_wrist && b.next_can_tie_choices().len() > 0 && b.hold
    }

    pub fn choice_tie(&self, ia : i32, ib : i32) -> Option<(Bound, bool, i32)> {
        let a = self.index(ia);
        let b = self.index(ib);
        let choices =b.next_can_tie_choices();
        let mut choice : Option<(Bound, bool, i32)> = None;
        let acc = a.tie_power();
        for ch in choices {
            let (bd, is_tie) = ch;
            let mut evd = if bd.is_upper() {b.anti_tie_upper()} else {b.anti_tie_lower()};
            if b.hold {
                evd = (evd - a.downforce()).max(0);
            }
            let hit = to_hit((acc - evd) * 10);
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

    pub fn tie_with_most_hit(&mut self, ia : i32, ib : i32) {
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
                    println!("{}{}", Board::title_front(), b.txt_attr());
                }
            }
        }
    }

    pub fn can_holddown(&self, ia : i32, ib : i32) -> bool {
        let a = self.index(ia);
        let b = self.index(ib);
        if a.fall {return false;}
        if b.hold {return false;}
        true
    }

    pub fn hit_holddown(&self, ia : i32, ib : i32) -> (i32, i32, i32) {
        let a = self.index(ia);
        let b = self.index(ib);
        let acc1 = a.acc_hand();
        let evd1 = b.evd_body();
        let hit1 = to_hit(50 + (acc1 - evd1) * 5);
        let acc2 = a.thrust();
        let evd2 = b.anti_thrust();
        let hit2 = to_hit(50 + (acc2 - evd2) * 5);
        let acc3 = b.thrust();
        let evd3 = a.anti_thrust();
        let hit3 = to_hit(50 + (acc3 - evd3) * 5    );
        (hit1, hit2, hit3)
    }

    pub fn holddown(&mut self, ia : i32, ib : i32) {
        let (hit1, hit2, hit3) = self.hit_holddown(ia, ib);
        println!("<hold down>");
        let hit_dice = self.dice.d(100);
        let is_hit1 = hit1 >= hit_dice;
        print!("{}", txt_hit("attach", hit1, hit_dice, is_hit1, "success"));
        if is_hit1 {
            let b = self.index(ib);
            if b.fall {
                self.index_mut(ib).hold = true;
                println!("  hold fallen opponent")
            } else {
                let hit_dice = self.dice.d(100);
                let is_hit2 = hit2 >= hit_dice;
                print!("{}", txt_hit("    push opponent", hit2, hit_dice, is_hit2, "success"));
                if is_hit2 {
                    let hit_dice = self.dice.d(100);
                    let is_hit3 = hit3 >= hit_dice;
                    
                    print!("{}", txt_hit("    pushed by opponent", hit3, hit_dice, is_hit3, "success"));
                    if is_hit3 {
                        print!("  both fall\n");
                        self.index_mut(ia).fall = true;
                        self.index_mut(ib).fall = true;
                    }else{
                        print!("  hold opponent\n");
                        self.index_mut(ib).fall = true;
                        self.index_mut(ib).hold = true;
                    }
                }
            }
        }
    }

    pub fn can_struggle(&self, ia : i32) -> bool {
        let a = self.index(ia);
        a.hold
    }

    pub fn hit_struggle(&self, ia : i32, ib : i32) -> i32 {
        let a = self.index(ia);
        let b = self.index(ib);
        let acc = a.anti_downforce();
        let evd = b.downforce();
        to_hit(50 + (acc - evd) * 5)
    }

    pub fn struggle(&mut self, ia : i32, ib : i32) {
        let hit = self.hit_struggle(ia, ib);
        println!("<struggle>");
        let hit_dice = self.dice.d(100);
        let is_hit = hit >= hit_dice;
        print!("{}", txt_hit("struggle", hit, hit_dice, is_hit, "success"));
        if is_hit {
            let mut a = self.index_mut(ia);
            a.hold = false;
            a.fall = !a.can_stand();
        }
    }

    pub fn can_stand(&self, ia : i32) -> bool {
        let a = self.index(ia);
        a.fall && !a.hold
    } 

    pub fn hit_stand(&self, ia : i32) -> i32 {
        let a = self.index(ia);
        if a.can_stand() {
            100
        }else{
            0
        }
    }

    pub fn stand(&mut self, ia : i32) {
        let a = self.index_mut(ia);
        if a.can_stand() {
            a.fall = false;
        }
        println!("<stant>\n")
    }
}