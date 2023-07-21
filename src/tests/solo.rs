use colorful::{Color, Colorful};

use crate::game::Game;

pub mod balance;
pub mod seq;

fn solo_thousand(str1 : i32, dex1 : i32, agi1 : i32, str2 : i32, dex2 : i32, agi2 : i32) {
    let mut win = 0;
    let mut draw = 0;
    let mut lose = 0;

    for seed in 0..1000 {
        let mut game = Game::new_solo_auto(seed, str1, dex1, agi1, str2, dex2, agi2);
        let result = game.main_auto();
        match result {
            Some(true) => win += 1,
            Some(false) => lose += 1,
            None => draw += 1,
        }
    }
    // println!("{str1} {dex1} {agi1} vs {str2} {dex2} {agi2}");
    println!("{} {} {} vs {} {} {}", str1.to_string().color(Color::Red), dex1.to_string().color(Color::Blue),agi1.to_string().color(Color::Green),str2.to_string().color(Color::Red),dex2.to_string().color(Color::Blue),agi2.to_string().color(Color::Green));
    println!("千局结果 : {} / {} / {}", win.to_string().color(Color::Green), draw.to_string().color(Color::Grey0), lose.to_string().color(Color::Red));
}




