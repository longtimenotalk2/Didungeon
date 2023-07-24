
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

#[derive(Clone, Debug)]
pub enum ChooseSkill {
    Pass,
    Wait,
    Skill {skill : Skill, it : Id, dir : Dir,},
    Move {pos : Pos, dir : Dir,},
}

#[derive(Clone, Debug)]
pub enum ChooseUnbound {
    Pass,
    Unbound(BoundPart),
}

#[derive(Clone, Debug)]
pub enum ChooseUntie {
    Pass,
    Untie(BoundPart),
}

#[derive(Clone, Debug)]
pub enum ChooseTie {
    Pass,
    Tight(BoundPart),
    Tie(BoundPart),
    Untie(BoundPart),
}

#[derive(Clone, Debug)]
pub enum Choose {
    Skill(ChooseSkill),
    Tie(ChooseTie),
    Unbound(ChooseUnbound),
    Untie(ChooseUntie),
}


#[derive(Clone, Debug)]
pub struct Return {
    choose : Option<Vec<Choose>>,
    winner : Option<bool>,
}

pub struct CtrlPara {
    pub need_show : bool,
    pub is_load : bool,
    pub force_auto : bool,
}

impl CtrlPara {
    pub fn new() -> Self {
        Self { need_show: true, is_load: false, force_auto: false }
    }
}


impl Return {
    fn new() -> Self {
        Self {
            choose: None,
            winner: None,
        }
    }

    fn new_with_choose(choose : Vec<Choose>) -> Self {
        Self {
            choose : Some(choose),
            winner: None,
        }
    }

    fn new_with_winner(is_ally_win : bool) -> Self {
        Self {
            choose: None,
            winner: Some(is_ally_win),
        }
    }

    pub fn winner(&self) -> Option<bool> {
        self.winner.clone()
    }

    pub fn get_choose(&self) -> Option<&Vec<Choose>> {
        self.choose.as_ref()
    }
}

impl Board {
    pub fn continue_turn(&mut self, para : CtrlPara) -> Return {
        if para.need_show {
            print!("{}", self.string_cache);
        }
        

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

    pub fn response_choose(&mut self, para : CtrlPara, choose : Choose) -> Return {
        if para.need_show {
            print!("{}", self.string_cache);
        }

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
