use crate::game::unit::Unit;

use super::Board;


impl Board {
    pub fn print(&self, act : Option<i32>) {
        println!("                  {}", Unit::title_1());
        println!("T ={:2}             {}", self.turn, Unit::title_2());
        for (i, unit) in self.units.iter().enumerate() {
            let ac = match act {
                None => " ",
                Some(a) => if i as i32 == a {">"} else {" "},
            };
            println!("{ac}{i} : {}{} {}", unit.txt_state(), unit.txt_bound(), unit.txt_attr());
        }
        
        println!("--------------------------------------------------------------------------");
    }

    pub fn title_front() -> String {
        let num = 18;
        let mut s = String::new();
        for _ in 0..num {
            s += " ";
        }
        s
    }
}