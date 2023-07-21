pub mod solo;

use std::fmt::Write;

use colorful::{Color, Colorful};

use crate::{game::{unit::{Unit, Dir}, board::Board}, common};

#[test]
fn test_find_target() {
    let mut board = Board::new(114514, 8);
    let id = 0;
    let pos = 3;
    board.insert_unit(Unit::new_noal(id, pos));
    board.insert_unit(Unit::new_any(1, 5, true));
    board.insert_unit(Unit::new_any(2, 6, false));
    board.show(None);

    let range = 3;
    let list = board.find_target_with_range(0, range);

    println!("{} (处于位置 {}) 距离 {} 以内的目标 : ", board.get_unit(0).identity(), pos, range);
    for (it, _) in list {
        print!("{}, ", board.get_unit(it).get_pos()) 
    }

}


#[test]
fn look_all_colors() {
    for color in Color::iterator() {
        print!("{}", "Sample".color(color.clone()));
        dbg!(&color);
    }
}

#[test]
#[ignore]
fn test_show_board() {
    let board = Board::new_noal_vs_kuinuo(114514);
    board.show(Some(0));
    // print!("\x1B[2J\x1B[1;1H");
}

#[test]
#[ignore]
fn test_show_bound() {
    // let s = "[\u{1b}[38;5;15m \u{1b}[0m]";
    let s = "".color(Color::Red);
    println!("{} {} {} {}", "💫", s, s, "A");
    println!("{} {} {} {}", "🧎", s, s, "A");
}

#[test]
#[ignore]
fn test_show_units() {
    let mut noel = Unit::new_noal(0, 0);
    let mut yelin = Unit::new_yelin(1, 2);
    let mut kuinuo = Unit::new_kuinuo(2, 1);

    noel.set_catch(2, Dir::Right);
    kuinuo.set_sleep(true);
    yelin.set_fall(true);

    Unit::show_title1();
    Unit::show_title2();
    noel.show(true);
    yelin.show(false);
    kuinuo.show(false);
    yelin.show(false);
}

#[test]
#[ignore]
fn test_serde() {
    // let a = Unit::new(0, "诺艾尔".to_string(), true, true, 0, 10, 10, 10);
    // // Convert the Point to a JSON string.
    // let serialized = serde_json::to_string(&a).unwrap();

    // // Prints serialized = {"x":1,"y":2}
    // println!("serialized = {}", serialized);
}

#[test]
#[ignore]
fn test_save() {
    let path = "assets/saves/save_0.ddg";
    let data = "hello2".to_string();
    let _result = common::save_file(path, data);
}

#[test]
#[ignore]
fn test_color_txt() {
    let mut s = String::new();
    write!(&mut s, "Hello {}", "World".to_string().color(Color::Yellow)).unwrap();
    println!("{}", s);
}