pub mod skill_list;

pub mod helper;

use serde::{Serialize, Deserialize};

use self::skill_list::{punch::Punch, catch::Catch};

use super::{board::Board, unit::{Id, Dir}};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Skill {
    Catch,
    Punch,
}

impl Skill {
    pub fn basic_set() -> Vec<Self> {
        vec!(Self::Catch, Self::Punch)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Skill::Catch => "✋擒拿",
            Skill::Punch => "🤜挥拳",
        }
    }

    fn create(&self) -> Box<dyn Skillize> {
        match self {
            Skill::Catch => Box::new(Catch::new()),
            Skill::Punch => Box::new(Punch::new()),
        }
    }

    pub fn get_targets(&self, board : &Board, id : Id) -> Vec<(Id, Dir)> {
        self.create().get_targets(board, id)
    }

    pub fn exe(&self, board : &mut Board, id : Id, it : Id, dir : &Dir) {
        self.create().exe(board, id, it, dir);
    }
}

pub trait Skillize {
    fn get_targets(&self, board : &Board, id : Id) -> Vec<(Id, Dir)>;
    fn exe(&self, board : &mut Board, id : Id, it : Id, dir : &Dir);
}