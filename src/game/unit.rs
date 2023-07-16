use serde::{Serialize, Deserialize};

use self::bound::BoundState;

pub mod bound;
pub mod new;
mod state;
mod attr;

type Id = u32;
type Pos = i32;

#[derive(Serialize, Deserialize, Debug)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    id : Id,
    name : String,
    ally : bool,
    pos : Pos,
    
    str_max : i32,
    dex_max : i32,
    agi_max : i32,
    inj_coefficient : i32,
    restore_rate : i32,

    bound : BoundState,
    fall : bool,
    stun : bool,
    sleep : bool,
    inj : i32,
    dir : Dir,
    action : bool,

    catch_left : Option<Id>,
    catch_right : Option<Id>,
    catched_left : Option<Id>,
    catched_right : Option<Id>,
}