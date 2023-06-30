use crate::game::unit::Unit;

pub mod game;
pub mod wyrand;

fn main() {
    println!("Hello, world!");
    println!("{}", Unit::title_1());
    println!("{}", Unit::title_2());
    let mut u = Unit::test_new2();
    println!("{}{}{}", u.txt_attr(), u.txt_bound(), u.txt_state());
    u.bound_ankle = true;
    println!("{}{}{}", u.txt_attr(), u.txt_bound(), u.txt_state());
    u.bound_wrist = true;
    println!("{}{}{}", u.txt_attr(), u.txt_bound(), u.txt_state());
    u.fall = true;
    println!("{}{}{}", u.txt_attr(), u.txt_bound(), u.txt_state());
    u.bound_neck = true;
    println!("{}{}{}", u.txt_attr(), u.txt_bound(), u.txt_state());
    u.bound_wrist = false;
    println!("{}{}{}", u.txt_attr(), u.txt_bound(), u.txt_state());


}
