use crate::{wyrand::Dice, game::skill::Skill};

use super::Board;

impl<'a> Board<'a> {
    pub fn anto_run(&mut self, turn : i32, dice : &mut Dice) {
        for _ in 0..turn {
            self.turn_pass();
            while let Some(actor) = self.find_next_actor() {
                self.action(actor, dice);
            }
            match self.is_defeated() {
                Some(true) => {
                    println!("enemy win!");
                    return;
                },
                Some(false) => {
                    println!("ally win!");
                    return;
                },
                None => (),
            }
        }
    }
}

impl<'a> Board<'a> {
    fn auto_make_choice(&self, ia : u8) -> Option<(Skill, u8)> {
        let mut choice : Option<(Skill, u8, i32)> = None;
        let mut txt = String::new();

        for skill in &Skill::all() {
            let target = self.target(&skill, ia);
            for ib in target {
                let (point, comment) = self.evaluate(skill, ia, ib);
                let name = skill.name();
                txt += &format!("{name}<{ib}> : {point}");
                if let Some(c) = comment {
                    txt += &format!( "({c})");
                }
                txt += "; ";
                match choice {
                    Some((_, _, point_)) => {
                        if point > point_ {
                            choice = Some((skill.clone(), ib, point));
                        }
                    },
                    None => choice = Some((skill.clone(), ib, point))
                }
            }
        }  

        println!("Choices : {txt}");
        let choice = match choice {
            Some((c, ib, _)) => Some((c, ib)),
            None => None,
        };
        choice
    }

    fn action(&mut self, ia : u8, dice : &mut Dice) {
        self.print(Some(ia));

        // try auto stand
        self.auto_stand(ia, dice);

        let skill = self.auto_make_choice(ia);
        match skill {
            Some((skl, ib)) => {
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

    fn is_defeated(&self) -> Option<bool> {
        let mut ally_defeated = true;
        let mut enemy_defeated = true;
        for u in &self.units {
            if !u.is_defeated() {
                match u.ally {
                    true => ally_defeated = ally_defeated && false,
                    false => enemy_defeated = ally_defeated && false,
                }
            }
        }
        if !ally_defeated && !enemy_defeated {
            None
        }else{
            Some(ally_defeated)
        }
    }

    fn turn_pass(&mut self) {
        self.turn += 1;
        for unit in &mut self.units {
            unit.end_turn();
        }
    }
}