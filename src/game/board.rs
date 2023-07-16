mod basic;
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
}