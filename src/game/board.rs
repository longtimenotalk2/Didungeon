use serde::{Serialize, Deserialize};

use super::unit::Unit;

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    units : Vec<Unit>,
}