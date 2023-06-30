use super::Board;

impl Board {
    fn action(&mut self, i : i32) {
        self.print(Some(i));
        if self.can_unbound(i){
            self.unbound(i);
        } else if self.can_tie(i, 1-i) {
                self.tie(i, 1-i)
        }else{
            println!("<pass>")
        }
        
    }

    pub fn solo_start(&mut self, turn : i32) {
        for _ in 0..turn {
            self.turn_pass();
            if self.index(0).spd() > self.index(1).spd() {
                self.action(0);
                self.action(1);
            }else{
                self.action(1);
                self.action(0);
            }
            if self.index(0).is_defeated() {
                println!("player 1 win!");
                return;
            }
            if self.index(1).is_defeated() {
                println!("player 0 win!");
                return;
            }
        }        
    }
}