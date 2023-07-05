use super::Board;

impl<'a> Board<'a> {
    pub fn find_melee_target(&self, ia : u8, mv : i32) -> Vec<u8> {
        let loc = self.get_location(ia);
        let mut ibs = vec!();
        let a = self.index(ia);

        // forward
        for f in 1..(mv+2) {
            let l = loc + f;
            if let Some(ib) = self.on_location(l) {
                let b = self.index(ib);
                if a.ally != b.ally {
                    ibs.push(ib);
                }
                if b.block() {
                    break;
                }
            }
        }
        // backward
        for f in 1..(mv+2) {
            let l = loc - f;
            if let Some(ib) = self.on_location(l) {
                let b = self.index(ib);
                if a.ally != b.ally {
                    ibs.push(ib);
                }
                if b.block() {
                    break;
                }
            }
        }
        ibs
    }
}

impl<'a> Board<'a> {
    fn get_location(&self, ia : u8) -> i32 {
        *self.locations.get(&ia).unwrap()
    }

    fn on_location(&self, loc : i32) -> Option<u8> {
        for (&i, &l) in self.locations.iter() {
            if loc == l {
                return Some(i);
            }
        }
        None
    }
}