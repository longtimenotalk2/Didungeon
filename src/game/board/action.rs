use crate::{game::{unit::Arrow, skill::Skillize}, wyrand::Dice};

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
                    if b.block() {
                        break;
                    }
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
                    if b.block() {
                        break;
                    }
                }
            }
        }
        ibs
    }

    pub fn rush_to(&mut self, ia : u8, ib : u8) -> Arrow {
        let loca = self.get_location(ia);
        let locb = self.get_location(ib);
        let (loc_new, direction, arr) = if loca > locb {
            (locb + 1, 1, Arrow::Up)
        }else{
            (locb - 1, -1, Arrow::Down)
        };
        self.set_location(ia, loc_new);
        let mut consider = ia;
        while let Some(ib) = self.coincide_with(consider) {
            consider = ib;
            self.set_location(ib, self.get_location(ib) + direction);
        }
        arr
    }
}

impl<'a> Board<'a> {
    fn get_location(&self, ia : u8) -> i32 {
        *self.locations.get(&ia).unwrap()
    }

    fn set_location(&mut self, ia : u8, loc : i32) {
        *self.locations.get_mut(&ia).unwrap() = loc;
    }

    fn coincide_with(&mut self, ia : u8) -> Option<u8> {
        let loc = self.get_location(ia);
        for ib in self.locations.keys() {
            if *ib != ia {
                if loc == self.get_location(*ib) {
                    return Some(*ib)
                }
            }
        }
        None
    }

    pub fn on_location(&self, loc : i32) -> Option<u8> {
        for (&i, &l) in self.locations.iter() {
            if loc == l {
                return Some(i);
            }
        }
        None
    }

    pub fn distance(&self, ia : u8, ib : u8) -> (i32, Arrow) {
        let loca = self.get_location(ia);
        let locb = self.get_location(ib);
        let dist = loca - locb;
        if dist > 0 {
            (dist, Arrow::Up)
        }else{
            (-dist, Arrow::Down)
        }
    }

    pub fn find_adjacent(&self, ia : u8) -> Vec<u8> {
        let loca = self.get_location(ia);
        let mut adjs = vec!();
        if let Some(ib) = self.on_location(loca - 1) {
            adjs.push(ib);
        }
        if let Some(ib) = self.on_location(loca + 1) {
            adjs.push(ib);
        }
        adjs
    }

    pub fn auto_stand(&mut self, ia : u8, dice : &mut Dice) {
        if self.anto_stand.target(self, ia).len() > 0 {
            print!("{}", self.anto_stand.exe(self, ia, ia, dice));
        }
    }

}