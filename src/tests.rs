

use crate::{game::{unit::{Unit, Dir}, board::Board}, common};




#[test]
#[ignore]
fn test_show_board() {
    let board = Board::new_noal_vs_kuinuo(114514);
    board.show(Some(0));
    // print!("\x1B[2J\x1B[1;1H");
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
    kuinuo.show(false);
    yelin.show(false);
}

#[test]
#[ignore]
fn test_serde() {
    let a = Unit::new(0, "诺艾尔".to_string(), true, true, 0, 10, 10, 10);
    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&a).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);
}

#[test]
#[ignore]
fn test_save() {
    let path = "assets/saves/save_0.ddg";
    let data = "hello2".to_string();
    let _result = common::save_file(path, data);
}