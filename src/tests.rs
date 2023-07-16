

use crate::game::unit::Unit;


#[test]
#[ignore]
fn test_serde() {
    let a = Unit::new(0, "诺艾尔".to_string(), true, 0);
    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&a).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

}