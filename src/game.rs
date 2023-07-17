use std::io;

use crate::common;

use self::board::{Board, turn::Command};

pub mod unit;
pub mod board;
pub mod skill;

pub struct Game {
    board : Board,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new_noal_vs_kuinuo(114514),
        }
    }

    pub fn main_loop(&mut self) {
        self.board.respond(Command::Continue);
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let strs: Vec<&str> = input.split_whitespace().collect();
            if strs.len() == 0 {
                self.board.respond(Command::Continue);
            }else{
                match strs[0] {
                    "save" => {
                        self.save();
                        println!("保存成功！");
                    },
                    "load" => {
                        self.load();
                        self.board.respond(Command::Continue);
                    },
                    _ => (),
                }
            }
        }
    }
}

impl Game {
    fn save(&self) {
        let save_path = "assets/saves/save_0.ddg";
        let serialized = serde_json::to_string(&self.board).unwrap();
        
        if let Err(any) = common::save_file(save_path, serialized) {
            println!("{}", any);
        }
    }

    fn load(&mut self) {
        let save_path = "assets/saves/save_0.ddg";
        
        match common::load_file(save_path) {
            Ok(s) => self.board = serde_json::from_str(&s).expect("Board struct changed! can not load!"),
            Err(any) => println!("{}", any),
        }
    }
}