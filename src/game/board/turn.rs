
use crate::game::unit::bound::BoundPart;
use crate::game::{skill::Skill, unit::{Id, Dir}};

use super::Board;
use super::Phase;

mod main;
mod start;
mod auto;
mod tie;

#[derive(Clone, Debug)]
pub enum ChooseSkill {
    Pass,
    Skill {skill : Skill, it : Id, dir : Dir,}
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
}


#[derive(Clone, Debug)]
pub struct Return {
    pub choose : Option<Vec<Choose>>,
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
        }
    }

    pub fn response_choose(&mut self, choose : Choose) -> Return {
        print!("{}", self.string_cache);

        match choose {
            Choose::Skill(skl) => self.response_main(skl),
            Choose::Tie(tie) => self.response_tie(tie),
        }
    }

    fn turn_end(&mut self) {
        self.turn += 1;
        for unit in &mut self.units {
            unit.end_turn()
        }
    }
}
