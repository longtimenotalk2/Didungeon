
use crate::game::unit::bound::BoundPart;
use crate::game::{skill::Skill, unit::{Id, Dir}};

use super::Board;
use super::Phase;

mod main;
mod start;
mod auto;
mod tie;
mod unbound;

#[derive(Clone, Debug)]
pub enum ChooseSkill {
    Pass,
    Skill {skill : Skill, it : Id, dir : Dir,}
}

#[derive(Clone, Debug)]
pub enum ChooseUnbound {
    Pass,
    Unbound(BoundPart),
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
}


#[derive(Clone, Debug)]
pub struct Return {
    choose : Option<Vec<Choose>>,
    winner : Option<bool>,
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
    pub fn continue_turn(&mut self) -> Return {
        print!("{}", self.string_cache);

        match self.phase {
            Phase::Start => self.turn_start(),
            Phase::Prepare { id } => self.turn_prepare(id),
            Phase::Tie { id, it, bound_point } => self.turn_tie(id, it, bound_point),
            Phase::Auto { id } => self.turn_auto(id),
            Phase::Main { id } => self.turn_main(id),
            Phase::Unbound { id, bound_point } => self.turn_unbound(id, bound_point),
            Phase::End {id} => self.turn_end(id),
        }
    }

    pub fn response_choose(&mut self, choose : Choose) -> Return {
        print!("{}", self.string_cache);

        match choose {
            Choose::Skill(skl) => self.response_main(skl),
            Choose::Tie(tie) => self.response_tie(tie),
            Choose::Unbound(ubd) => self.response_unbound(ubd),
        }
    }
}
