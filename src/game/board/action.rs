use crate::game::unit::Arrow;

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

    fn find_arr_with(&self, ia : u8, arr : &Arrow) -> Option<u8> {
        let mut loc = self.get_location(ia);
        match arr {
            Arrow::Up => {loc -= 1;},
            Arrow::Down => {loc += 1;},
        }
        self.on_location(loc)
    }

    pub fn hold(&mut self, ia : u8, arr : &Arrow) {
        match self.find_arr_with(ia, arr) {
            Some(ib) => {
                let b = self.index(ib);
                if let Some(ic) = &b.hold {
                    self.index_mut(*ic).catch = None;
                }
                let b = self.index_mut(ib);
                b.hold = Some(ia);
                let a = self.index_mut(ia);
                a.catch = Some(ib);
            },
            None => (),
        }
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

    pub fn catch_return_from(&mut self, ia : u8) {
        if let Some(ib) = &self.index(ia).catch {
            self.rush_to(*ib, ia);
        }
        if let Some(ib) = &self.index(ia).hold {
            self.rush_to(*ib, ia);
        }
    }
}