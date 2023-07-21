use std::io;

use crate::common;

use self::board::Board;

pub mod unit;
pub mod board;
pub mod skill;

pub struct Game {
    board : Board,
    history : Vec<(Board, bool)>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new_noal_vs_kuinuo(114514),
            history : vec![],
        }
    }

    pub fn main_loop(&mut self) {
        let mut result = self.board.continue_turn();

        loop  {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let strs: Vec<&str> = input.split_whitespace().collect();

            if strs.len() > 0 {
                match strs[0] {
                    "save" => {
                        self.save();
                        println!("保存成功！");
                        continue;
                    },
                    "load" => {
                        self.load();
                        self.history = vec!();
                        result = self.board.continue_turn();
                        continue;
                    },
                    "undo" => {
                        while let Some((b, is_choose)) = self.history.pop() {
                            if is_choose {
                                self.board = b;
                                result = self.board.continue_turn();
                                break;
                            }
                        }
                        continue;
                    }
                    "back" => {
                        if let Some((b, _)) = self.history.pop() 
                        {
                            self.board = b;
                            result = self.board.continue_turn();
                        }else{
                            println!("初始状态，撤销失败");
                        }
                        continue;
                    }
                    _ => (),
                }
            }

            match &result.choose {
                Some(chooses) => {
                    if strs.len() == 0 {
                        println!("请输入选项对应的数字！");
                    }else{
                        let i = strs[0];
                        match i.parse::<usize>() {
                            Err(_) => println!("请输入一个自然数！"),
                            Ok(i) => {
                                let num = chooses.len();
                                if i > num {
                                    println!("数值越界！")
                                }else {
                                    self.history.push((self.board.clone(), true));
                                    result = self.board.response_choose(chooses[i].clone());
                                }
                            }, 
                        }
                    }
                },
                None => {
                    self.history.push((self.board.clone(), false));
                    self.board.set_to_start();
                    result = self.board.continue_turn();
                },
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