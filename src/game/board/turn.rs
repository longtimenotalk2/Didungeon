
use serde::{Deserialize, Serialize};

use crate::game::unit::Pos;
use crate::game::unit::bound::BoundPart;
use crate::game::{skill::Skill, unit::{Id, Dir}};

use super::Board;
use super::Phase;

mod main;
mod start;
mod auto;
mod tie;
mod unbound;
mod untie;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChooseSkill {
    Pass,
    Wait,
    Skill {skill : Skill, it : Id, dir : Dir,},
    Move {pos : Pos, dir : Dir,},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChooseUnbound {
    Pass,
    Unbound(BoundPart),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChooseUntie {
    Pass,
    Untie(BoundPart),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChooseTie {
    Pass,
    Tight(BoundPart),
    Tie(BoundPart),
    Untie(BoundPart),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Choose {
    Skill(ChooseSkill),
    Tie(ChooseTie),
    Unbound(ChooseUnbound),
    Untie(ChooseUntie),
}




pub struct CtrlPara {
    pub force_auto : bool,
    pub printer : Option<Printer>,
}

impl CtrlPara {
    pub fn new() -> Self {
        Self { 
            force_auto: false,
            printer: None,
        }
    }

    pub fn new_with_printer() -> Self {
        Self { 
            force_auto: false,
            printer: Some(Printer::new()),
        }
    }

    pub fn show_cache(&self) {
        if let Some(p) = &self.printer {
            println!("{}", p.cache);
        }
    }

    pub fn show_temp(&self) {
        if let Some(p) = &self.printer {
            println!("{}", p.temp);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Return {
    pub choose : Option<(Vec<Choose>, Choose)>,
    pub winner : Option<bool>,
}


impl Return {
    fn new() -> Self {
        Self {
            choose: None,
            winner: None,
        }
    }

    fn new_with_choose_and_default(choose : Vec<Choose>, default : Choose) -> Self {
        Self {
            choose : Some((choose, default)),
            winner: None,
        }
    }

    fn new_with_winner(is_ally_win : bool) -> Self {
        Self {
            choose: None,
            winner: Some(is_ally_win),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Printer {
    cache : String,
    temp : String,
}

impl Printer {
    pub fn new() -> Self {
        Self {
            cache: String::new(),
            temp: String::new(),
        }
    }

    pub fn show_cache(&self) {
        println!("{}", self.cache)
    }

    pub fn show_temp(&self) {
        println!("{}", self.temp)
    }
}

impl Board {
    pub fn continue_turn(&mut self, para : &mut CtrlPara) -> Return {        
        match self.phase {
            Phase::Start => self.turn_start(para),
            Phase::Prepare { id } => self.turn_prepare(para, id),
            Phase::Tie { id, it, bound_point } => self.turn_tie(para, id, it, bound_point),
            Phase::Auto { id } => self.turn_auto(para, id),
            Phase::Main { id, can_wait } => self.turn_main(para, id, can_wait),
            Phase::Unbound { id, bound_point } => self.turn_unbound(para, id, bound_point),
            Phase::Untie { id, it, bound_point } => self.turn_untie(para, id, it, bound_point),
            Phase::Wait { id } => self.turn_wait(para, id),
            Phase::End {id} => self.turn_end(para, id),
        }
    }

    pub fn response_choose(&mut self, para : &mut CtrlPara, choose : Choose) -> Return {
        match choose {
            Choose::Skill(skl) => self.response_main(para, skl),
            Choose::Tie(tie) => self.response_tie(para, tie),
            Choose::Unbound(ubd) => self.response_unbound(para, ubd),
            Choose::Untie(ubd) => self.response_untie(para, ubd),
        }
    }

    pub fn get_turn(&self) -> i32 {
        self.turn
    }
}
