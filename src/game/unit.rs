use serde::{Serialize, Deserialize};

use self::bound::BoundState;

use super::skill::Skill;

pub mod bound;
pub mod new;
mod state;
mod attr;
mod show;
mod basic;
mod action;

pub type Id = u32;
pub type Pos = i32;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
}

impl Dir {
    pub fn all() -> Vec<Self> {
        vec!(Self::Left, Self::Right)
    }

    pub fn notice(&self) -> &'static str {
        match self {
            Dir::Left => "↑",
            Dir::Right => "↓",
        }
    }

    pub fn anti(&self) -> Dir {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unit {
    id : Id,
    name : String,
    name_fix_length : String,
    ally : bool,
    you : bool,
    pos : Pos,
    skills : Vec<Skill>,
    
    str_max : i32,
    dex_max : i32,
    agi_max : i32,
    inj_coefficient : i32,
    restore_rate : i32,

    bound : BoundState,
    fall : bool,
    stun : bool,
    sleep : bool,
    shock : bool,
    inj : i32,
    dir : Dir,
    action : bool,
    wait : bool,

    catch_left : Option<Id>,
    catch_right : Option<Id>,
    catched_left : Option<Id>,
    catched_right : Option<Id>,
}