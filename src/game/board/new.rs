use std::collections::HashMap;

use colorful::{Color, Colorful};

use crate::{game::unit::Unit, wyrand::Dice};

use super::{Board, Phase};

impl Board {
    pub fn new_team(seed : u64) -> Self {
        let agi1 = 14;
        let dex1 = 14;
        let str1 = 14;
        let agi2 = 14;
        let dex2 = 14;
        let str2 = 14;

        let color1 = Color::Blue;
        let color2 = Color::Red;
        let name1 = "甲".to_string().color(color1).to_string();
        let name2 = "乙".to_string().color(color2).to_string();

        let name1f = "甲      ".to_string().color(color1).to_string();
        let name2f = "乙      ".to_string().color(color2).to_string();
        
        let unit1 = Unit::new(0, name1, name1f, true, true, 0, str1, dex1, agi1);
        let unit2 = Unit::new(1, name2, name2f, false, false, 3, str2, dex2, agi2);
    
        let mut board = Self::new(seed, 8);
        board.insert_unit(unit1);
        board.insert_unit(unit2);
        board
    }



    pub fn new_solo(seed : u64, is_you : bool, str1 : i32, dex1 : i32, agi1 : i32, str2 : i32, dex2 : i32, agi2 : i32) -> Self {
        let color1 = Color::Blue;
        let color2 = Color::Red;
        let name1 = "甲".to_string().color(color1).to_string();
        let name2 = "乙".to_string().color(color2).to_string();

        let name1f = "甲      ".to_string().color(color1).to_string();
        let name2f = "乙      ".to_string().color(color2).to_string();
        
        let unit1 = Unit::new(0, name1, name1f, true, is_you, 0, str1, dex1, agi1);
        let unit2 = Unit::new(1, name2, name2f, false, false, 1, str2, dex2, agi2);
    
        let mut board = Self::new(seed, 2);
        board.insert_unit(unit1);
        board.insert_unit(unit2);
        board
    }

    pub fn new_noal_vs_kuinuo(seed : u64) -> Self {
        let mut board = Self::new(seed, 2);
        board.insert_unit(Unit::new_noal_bound(0, 0));
        board.insert_unit(Unit::new_kuinuo(1, 1));
        board
    }

    pub fn new(seed : u64, length : i32) -> Self {
        Self {
            indexs: HashMap::new(),
            units: vec!(),
            dice: Dice::new(seed),
            pos_min: 0,
            pos_length: length,
            turn : 1,
            phase : Phase::Start,
            string_cache: String::new(),
            acted_ids: vec![],
        }
    }

    pub fn insert_unit(&mut self, unit : Unit) {
        let id = unit.get_id();
        self.units.push(unit);
        let index_now = self.units.len() - 1;
        self.indexs.insert(id, index_now);
    }
}