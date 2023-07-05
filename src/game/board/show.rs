use crate::game::unit::Unit;

use super::Board;


impl<'a> Board<'a> {
    pub fn print(&self, act : Option<u8>) {
        println!("[T = {:4<}]            {}", self.turn, Unit::title_1());
        println!("                    {}", Unit::title_2());
        for (i, unit) in self.units.iter().enumerate() {
            let ac = match act {
                None => " ",
                Some(a) => if i as u8 == a {">"} else {" "},
            };
            println!("{ac}{i}{}: {}{} {}", unit.txt_ally(), unit.txt_state(), unit.txt_bound(), unit.txt_attr());
        }
        
        println!("---------------------------------------------------------------------------------");
    }

    pub fn title_front() -> String {
        let num = 20;
        let mut s = String::new();
        for _ in 0..num {
            s += " ";
        }
        s
    }
}