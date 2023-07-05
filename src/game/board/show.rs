use crate::game::unit::Unit;

use super::Board;


impl<'a> Board<'a> {
    pub fn print(&self, act : Option<u8>) {
        println!("[T = {:4<}]            {}", self.turn, Unit::title_1());
        println!("                    {}", Unit::title_2());
        let min = self.locations.values().min().unwrap();
        let max = self.locations.values().max().unwrap();
        for loc in (*min)..(*max + 1) {
            if let Some(id) = self.on_location(loc) {
                let ac = match act {
                    None => " ",
                    Some(a) => if id as u8 == a {">"} else {" "},
                };
                let unit = self.index(id);
                println!("{ac}{id}{}: {}{} {}", unit.txt_ally(), unit.txt_state(), unit.txt_bound(), unit.txt_attr());
            }else{
                println!("[Void]");
            }
        }

        // for (i, unit) in self.units.iter().enumerate() {
        //     let ac = match act {
        //         None => " ",
        //         Some(a) => if i as u8 == a {">"} else {" "},
        //     };
        //     println!("{ac}{i}{}: {}{} {}", unit.txt_ally(), unit.txt_state(), unit.txt_bound(), unit.txt_attr());
        // }
        
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