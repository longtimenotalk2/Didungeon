use crate::game::unit::Unit;

pub mod game;

fn main() {
    println!("Hello, world!");
    let u = Unit::test_new1();
    println!("{}", u.txt_bound());
}
