mod basic;
mod find;
pub mod new;
mod show;
pub mod turn;
mod action;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::wyrand::Dice;

use super::unit::{Unit, Id};

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    indexs : HashMap<Id, usize>,
    units : Vec<Unit>,
    dice : Dice,
    pos_min : i32,
    pos_length : i32,
    turn : i32,
    temp_actor_now : Option<Id>,
    temp_target_now : Option<Id>,
    temp_remained_bound_value : i32,
}