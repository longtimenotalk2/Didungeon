use super::Board;

impl Board {
    pub fn print(&self, act : Option<i32>) {
        println!("==========================================");
        println!("T ={:2}             StrDexEvdPowAccEvdInj", self.turn);
        for (i, unit) in self.units.iter().enumerate() {
            let ac = match act {
                None => " ",
                Some(a) => if i as i32 == a {">"} else {" "},
            };
            println!("{ac}{i} : {}{} {}", unit.txt_state(), unit.txt_bound(), unit.txt_attr());
        }
        
        println!("-----------------------------------------");
    }
}