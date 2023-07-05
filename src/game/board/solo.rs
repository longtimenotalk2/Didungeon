use crate::{wyrand::Dice, game::skill::Skill};

use super::Board;

impl<'a> Board<'a> {
    fn make_choice(&self, ia : u8) -> Option<Skill> {
        let mut choice : Option<(Skill, i32)> = None;
        let ib = 1-ia;
        let mut txt = String::new();
        let mut matcher = |skill : Skill, point : i32, comment : Option<String>| {
            let name = skill.name();
            txt += &format!("{name} : {point}");
            if let Some(c) = comment {
                txt += &format!( "({c})");
            }
            txt += "; ";
            match choice {
                Some((_, point_)) => {
                    if point > point_ {
                        choice = Some((skill, point));
                    }
                },
                None => choice = Some((skill, point))
            }
        };
        for skl in Skill::all() {
            if self.can(&skl, ia, ib) {
                let (point, txt) = self.evaluate(&skl, ia, ib);
                matcher(skl, point, txt)
            }
        }
        println!("Choices : {txt}");
        let choice = match choice {
            Some((c, _)) => Some(c),
            None => None,
        };
        choice
    }

    fn action(&mut self, ia : u8, dice : &mut Dice) {
        self.print(Some(ia));
        let ib = 1-ia;
        let mut a = self.index_mut(ia);
        if a.stun {
            a.stun = false;
            a.action = false;
            println!("<awake>");
            return;
        }
        if a.fall && a.can_stand() && !a.hold {
            a.fall = false;
            println!("<anto stand>");
        }
        let skill = self.make_choice(ia);
        match skill {
            Some(skl) => {
                let txt = self.exe(&skl, ia, ib, dice);
                print!("{txt}")
            }
            None => println!("pass")
        }
        self.index_mut(ia).action = false;
    }

    fn find_next_actor(&self) -> Option<u8> {
        let mut next : Option<(u8, i32)> = None;
        for (i, p) in self.units.iter().enumerate() {
            let i : u8 = i.try_into().unwrap();
            if p.action {
                let spd = p.spd();
                match next {
                    Some((_, s)) => {
                        if spd > s {
                            next = Some((i, spd));
                        }
                    },
                    None => {next = Some((i, spd));},
                }
            }
        }
        match next {
            Some((i, _)) => Some(i),
            None => None,
        }
    }

    pub fn anto_run(&mut self, turn : i32, dice : &mut Dice) {
        for _ in 0..turn {
            self.turn_pass();
            while let Some(actor) = self.find_next_actor() {
                self.action(actor, dice);
            }
            if self.index(0).is_defeated() {
                println!("player 1 win!");
                return;
            }
            if self.index(1).is_defeated() {
                println!("player 0 win!");
                return;
            }
        }
    }

    // fn make_choice(&self, i : i32) -> Option<Action> {
    //     let mut choice : Option<(Action, i32)> = None;
    //     let ia = i;
    //     let ib = 1-i;
    //     let mut txt = String::new();
    //     let mut matcher = |a : (Action, i32)| {
    //         let hit = a.1;
    //         match choice {
    //             Some((_, hit_)) => {
    //                 if hit > hit_ {
    //                     choice = Some(a);
    //                 }
    //             },
    //             None => choice = Some(a),
    //         }
    //     };
    //     for act in Action::all() {
    //         match act {
    //             Action::Unbound => {
    //                 if self.can_unbound(ia) {
    //                     let hit = self.hit_unbound(ia);
    //                     txt += &format!("unbound : {hit}, ");
    //                     matcher((Action::Unbound, hit))
    //                 }
    //             },
    //             Action::Tie => {
    //                 if self.can_tie(ia, ib) {
    //                     if let Some((_, _, hit)) = self.choice_tie(ia, ib) {
    //                         txt += &format!("tie : {hit}, ");
    //                         matcher((Action::Tie, hit))
    //                     }
    //                 }
    //             },
    //             Action::Holddowm => {
    //                 if self.can_holddown(ia, ib) {
    //                     let (hit1, hit2, hit3) = self.hit_holddown(ia, ib);
    //                     let hit = hit1 * (hit2 * hit3) / 10000;
    //                     txt += &format!("holddown : {hit}, ");
    //                     matcher((Action::Holddowm, hit))
    //                 }
    //             },
    //             Action::Struggle => {
    //                 if self.can_struggle(ia) {
    //                     let hit = self.hit_struggle(ia, ib);
    //                     txt += &format!("struggle : {hit}, ");
    //                     matcher((Action::Struggle, hit))
    //                 }
    //             },
    //             Action::Punch => {
    //                 if self.can_punch(ia, ib) {
    //                     let (hit, dmg) = self.hit_and_dmg_punch(ia, ib);
    //                     let point = (hit * dmg / (dmg + 1)).min(80);
    //                     txt += &format!("punch : {point}({hit}% * {dmg}), ");
    //                     matcher((Action::Punch, point))
    //                 }
    //             },
    //         }
    //     }
    //     println!("Choices : {txt}");
    //     let choice = match choice {
    //         Some((c, _)) => Some(c),
    //         None => None,
    //     };
    //     choice
    // }

    // fn action(&mut self, i : i32) {
    //     self.print(Some(i));
    //     let ia = i;
    //     let ib = 1-i;
    //     if self.index(ia).stun {
    //         self.index_mut(ia).stun = false;
    //         println!("<stun remove>");
    //         return;
    //     }
    //     if self.can_auto_stand(ia) {
    //         println!("<stand auto>");
    //         self.index_mut(ia).fall = false;
    //     }
    //     match self.make_choice(i) {
    //         Some(act) => match act {
    //             Action::Unbound => self.unbound(ia),
    //             Action::Tie => self.tie_with_most_hit(ia, ib),
    //             Action::Holddowm => self.holddown(ia, ib),
    //             Action::Struggle => self.struggle(ia, ib),
    //             Action::Punch => self.punch(ia, ib),
    //         },
    //         None => println!("<Pass>")
    //     }
    //     self.index_mut(ia).action = false; 
    // }

    // pub fn solo_start(&mut self, turn : i32) {
    //     for _ in 0..turn {
    //         self.turn_pass();
    //         if self.index(0).spd() >= self.index(1).spd() {
    //             self.action(0);
    //             if self.index(1).action {
    //                 self.action(1);
    //             }
                
    //         }else{
    //             self.action(1);
    //             if self.index(0).action {
    //                 self.action(0);
    //             }
    //         }
    //         if self.index(0).is_defeated() {
    //             println!("player 1 win!");
    //             return;
    //         }
    //         if self.index(1).is_defeated() {
    //             println!("player 0 win!");
    //             return;
    //         }
    //     }
    // }
}