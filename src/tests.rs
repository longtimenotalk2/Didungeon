

use crate::game::unit::{Unit, Dir};


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
fn test_show_units() {
    let mut noel = Unit::new_noal(0, 0);
    let mut yelin = Unit::new_yelin(1, 2);
    let mut kuinuo = Unit::new_kuinuo(2, 1);

    noel.set_catch(2, Dir::Right);
    kuinuo.set_sleep(true);
    yelin.set_fall(true);

    let intend = 24;
    Unit::show_title1(intend);
    Unit::show_title2(intend);
    noel.show();
    kuinuo.show();
    yelin.show();
}