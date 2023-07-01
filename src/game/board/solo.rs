use super::Board;

enum Action {
    Unbound,
    Tie,
    Holddowm,
    Struggle,
    Stand,
}

impl Action {
    fn all() -> Vec<Self> {
        vec!(
            Self::Unbound, 
            Self::Struggle,
            Self::Holddowm,
            Self::Tie,
            Self::Stand,
        )
    }
}

impl Board {

    fn make_choice(&self, i : i32) -> Option<Action> {
        let mut choice : Option<(Action, i32)> = None;
        let ia = i;
        let ib = 1-i;
        let mut txt = String::new();
        for act in Action::all() {
            match act {
                Action::Unbound => {
                    if self.can_unbound(ia) {
                        let hit = self.hit_unbound(ia);
                        txt += &format!("unbound : {hit}, ");
                        match choice {
                            Some((_, hit_)) => {
                                if hit > hit_ {
                                    choice = Some((Action::Unbound, hit));
                                }
                            },
                            None => choice = Some((Action::Unbound, hit)),
                        }
                    }
                },
                Action::Tie => {
                    if self.can_tie(ia, ib) {
                        if let Some((_, _, hit)) = self.choice_tie(ia, ib) {
                            txt += &format!("tie : {hit}, ");
                            match choice {
                                Some((_, hit_)) => {
                                    if hit > hit_ {
                                        choice = Some((Action::Tie, hit));
                                    }
                                },
                                None => choice = Some((Action::Tie, hit)),
                            }
                        }
                    }
                },
                Action::Holddowm => {
                    if self.can_holddown(ia, ib) {
                        let (hit1, hit2, hit3) = self.hit_holddown(ia, ib);
                        let hit = hit1 * (hit2 * (100 - hit3)) / 10000;
                        txt += &format!("holddown : {hit}, ");
                        match choice {
                            Some((_, hit_)) => {
                                if hit > hit_ {
                                    choice = Some((Action::Holddowm, hit));
                                }
                            },
                            None => choice = Some((Action::Holddowm, hit)),
                        }
                    }
                },
                Action::Struggle => {
                    if self.can_struggle(ia) {
                        let hit = self.hit_struggle(ia, ib);
                        txt += &format!("struggle : {hit}, ");
                        match choice {
                            Some((_, hit_)) => {
                                if hit > hit_ {
                                    choice = Some((Action::Struggle, hit));
                                }
                            },
                            None => choice = Some((Action::Struggle, hit)),
                        }
                    }
                },
                Action::Stand => {
                    if self.can_stand(ia) {
                        let hit = self.hit_stand(ia);
                        txt += &format!("stand : {hit}, ");
                        match choice {
                            Some((_, hit_)) => {
                                if hit > hit_ {
                                    choice = Some((Action::Stand, hit));
                                }
                            },
                            None => choice = Some((Action::Stand, hit)),
                        }
                    }
                },
                
            }
        }
        println!("Choices : {txt}");
        let choice = match choice {
            Some((c, _)) => Some(c),
            None => None,
        };
        choice
    }

    fn action(&mut self, i : i32) {
        self.print(Some(i));
        let ia = i;
        let ib = 1-i;
        match self.make_choice(i) {
            Some(act) => match act {
                Action::Unbound => self.unbound(ia),
                Action::Tie => self.tie_with_most_hit(ia, ib),
                Action::Holddowm => self.holddown(ia, ib),
                Action::Struggle => self.struggle(ia, ib),
                Action::Stand => self.stand(ia),
            },
            None => println!("<Pass>")
        }
        
    }

    pub fn solo_start(&mut self, turn : i32) {
        for _ in 0..turn {
            self.turn_pass();
            if self.index(0).spd() > self.index(1).spd() {
                self.action(0);
                self.action(1);
            }else{
                self.action(1);
                self.action(0);
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
}