mod basic;
mod find;
pub mod new;
mod show;
pub mod turn;
mod moving;
mod action;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::wyrand::Dice;

use super::unit::{Unit, Id};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Phase {
    Start,
    Prepare {id : Id},
    Tie {id : Id, it : Id, bound_point : i32},
    Auto {id : Id},
    Main {id : Id, can_wait : bool}, 
    Unbound {id : Id, bound_point : i32},
    Untie {id : Id, it : Id, bound_point : i32},
    Wait {id : Id},
    End {id : Id},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    indexs : HashMap<Id, usize>,
    units : Vec<Unit>,
    dice : Dice,
    pos_min : i32,
    pos_length : i32,
    turn : i32,
    pub phase : Phase,
    acted_ids : Vec<Id>,
}

