pub mod skill_list;

pub mod helper;

use serde::{Serialize, Deserialize};

use self::skill_list::{punch::Punch, catch::Catch, unbound::Unbound};

use super::{board::Board, unit::{Id, Dir}};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Skill {
    Unbound,
    Catch,
    Punch,
}

impl Skill {
    pub fn basic_set() -> Vec<Self> {
        vec!(
            Self::Unbound,
            Self::Catch, 
            Self::Punch,
        )
    }

    pub fn name(&self) -> &'static str {
        match self {
            Skill::Unbound => "⚡挣脱",
            Skill::Catch => "✋擒拿",
            Skill::Punch => "🤜挥拳",
        }
    }

    fn create(&self) -> Box<dyn Skillize> {
        match self {
            Skill::Unbound => Box::new(Unbound::new()),
            Skill::Catch => Box::new(Catch::new()),
            Skill::Punch => Box::new(Punch::new()),
        }
    }

    pub fn get_targets(&self, board : &Board, id : Id) -> Vec<(Id, Dir)> {
        self.create().get_targets(board, id)
    }

    pub fn exe(&self, s: &mut String, board : &mut Board, id : Id, it : Id, dir : &Dir) {
        self.create().exe(s, board, id, it, dir);
    }

    pub fn choice_show(&self, board : &Board, id : Id, it : Id, dir : &Dir) -> String {
        self.create().choice_show(board, id, it, dir)
    }

    pub fn analyse(&self, board : &Board, id : Id, it : Id, dir : &Dir) -> Board {
        self.create().analyse(board, id, it, dir)
    }
}

pub trait Skillize {
    fn get_targets(&self, board : &Board, id : Id) -> Vec<(Id, Dir)>;
    fn exe(&self, s : &mut String, board : &mut Board, id : Id, it : Id, dir : &Dir);
    fn choice_show(&self, board : &Board, id : Id, it : Id, dir : &Dir) -> String;
    fn analyse(&self, board : &Board, id : Id, it : Id, dir : &Dir) -> Board;
}