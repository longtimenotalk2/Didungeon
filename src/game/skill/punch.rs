pub struct Punch {
    basic_hit : i32,
    hit_rate : i32,
    basic_dmg : i32,
}

impl Punch {
    pub fn new() -> Self {
        Self {
            basic_hit: 50,
            hit_rate: 5,
            basic_dmg: 10,
        }
    }
}
