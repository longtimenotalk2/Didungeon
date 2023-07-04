pub mod punch;

use std::collections::HashMap;

use crate::wyrand::Dice;

use self::punch::Punch;

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

pub trait Skillize {
    fn can(&self) -> Box<dyn Fn(&Board, u8, Option<u8>) -> bool>;
    fn exe(&self) -> Box<dyn FnMut(&mut Board, u8, Option<u8>, &mut Dice) -> String + '_>;
}

#[derive(PartialEq, Eq, Hash)]
pub enum Skill {
    Punch,
}

impl Skill {
    pub fn all() -> Vec<Self> {
        vec!(Self::Punch)
    }

    pub fn all_data() -> HashMap<Self, Box<dyn Skillize>> {
        let mut hash :HashMap<Self, Box<dyn Skillize>> = HashMap::new();
        for skill in Self::all() {
            match skill {
                Skill::Punch => {hash.insert(skill, Box::new(Punch::new()));},
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

    pub fn get_can(&self, skill : &Skill) -> Box<dyn Fn(&Board, u8, Option<u8>) -> bool> {
        self.skill_data.get(skill).unwrap().can()
    }

    pub fn get_exe(&self, skill : &Skill) -> Box<dyn FnMut(&mut Board, u8, Option<u8>, &mut Dice) -> String + '_> {
        self.skill_data.get(skill).unwrap().exe()
    }
}