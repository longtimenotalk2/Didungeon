mod punch;
mod tie;
// mod hold;
mod struggle;
mod unbound;

use std::collections::HashMap;

use crate::wyrand::Dice;

use self::{punch::Punch, tie::Tie, struggle::Struggle, unbound::Unbound};

use super::board::Board;

const BASIC_HIT : i32 = 50;
const HIT_RATE : i32 = 5;

fn to_hit(h : i32) -> i32 {
    h.max(0).min(100)
}

fn to_dmg(dmg_o : i32, min_dmg_set : i32) -> i32 {
    dmg_o.max(min_dmg_set)
}

fn txt_hit(target : &str, hit : i32, hit_dice : i32, is_hit : bool, success : &str) -> String {
    format!("  {target} : {hit} -> d100 = {hit_dice} -> {}\n", if is_hit {success} else {"miss"})
}

fn txt_announce(skill : &Skill, ib : u8) -> String {
    format!("<{} -> {}>\n", skill.name(), ib)
}

pub trait Skillize {
    fn target(&self, board : &Board, ia : u8) -> Vec<u8>;
    fn evaluate(&self, board : &Board, ia : u8, ib : u8) -> (i32, Option<String>);
    fn exe(&self, board : &mut Board, ia : u8, ib : u8, dice : &mut Dice) -> String;
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Skill {
    Punch,
    Struggle,
    Tie, 
    Unbound,
}

impl Skill {    
    pub fn name(&self) -> &str {
        match self {
            Skill::Punch => "punch",
            Skill::Struggle => "struggle",
            Skill::Tie => "tie",
            Skill::Unbound => "unbound"
        }
    }

    pub fn all() -> Vec<Self> {
        vec!(
            Self::Punch,
            Self::Struggle,
            Self::Tie,
            Self::Unbound,
        )
    }

    pub fn all_data() -> HashMap<Self, Box<dyn Skillize>> {
        let mut hash :HashMap<Self, Box<dyn Skillize>> = HashMap::new();
        for skill in Self::all() {
            match skill {
                Skill::Punch => {
                    hash.insert(skill, Box::new(Punch::new()));
                },
                Skill::Struggle => {
                    hash.insert(skill, Box::new(Struggle::new()));
                },
                Skill::Tie => {
                    hash.insert(skill, Box::new(Tie::new()));
                },
                Skill::Unbound => {
                    hash.insert(skill, Box::new(Unbound::new()));
                },
            }
        }
        hash
    }
}

pub struct SkillSet {
    skill_data : HashMap<Skill, Box<dyn Skillize>>,
}

impl SkillSet {
    pub fn new() -> Self {
        Self {
            skill_data : Skill::all_data(),
        }
    }

    pub fn target(&self, skill : &Skill, board : &Board, ia : u8) -> Vec<u8> {
        self.skill_data.get(skill).unwrap().target(board, ia)
    }

    pub fn evaluate(&self, skill : &Skill, board : &Board, ia : u8, ib : u8) -> (i32, Option<String>) {
        self.skill_data.get(skill).unwrap().evaluate(board, ia, ib)
    }

    pub fn exe(&self, skill : &Skill, board : &mut Board, ia : u8, ib : u8, dice : &mut Dice) -> String {
        self.skill_data.get(skill).unwrap().exe(board, ia, ib, dice)
    }
}