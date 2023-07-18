mod basic;
mod find;
pub mod new;
mod show;
pub mod turn;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::wyrand::Dice;

use super::unit::{Unit, Id};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Phase {
    Start,
    Prepare {id : Id},
    Tie {id : Id, it : Id, bound_point : i32},
    Auto {id : Id},
    Main {id : Id}, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    indexs : HashMap<Id, usize>,
    units : Vec<Unit>,
    dice : Dice,
    pos_min : i32,
    pos_length : i32,
    turn : i32,
    phase : Phase,
}

