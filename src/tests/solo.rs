use colorful::{Color, Colorful};

use crate::game::Game;

#[test]
fn test_solo_hundred() {
    let str1 = 10;
    let dex1 = 10;
    let agi1 = 10;
    let str2 = 10;
    let dex2 = 10;
    let agi2 = 10;

    let mut win = 0;
    let mut draw = 0;
    let mut lose = 0;

    for seed in 0..100 {
        let mut game = Game::new_solo_auto(seed, str1, dex1, agi1, str2, dex2, agi2);
        let result = game.main_auto();
        match result {
            Some(true) => win += 1,
            Some(false) => lose += 1,
            None => draw += 1,
        }
    }

    println!("百局结果 : {} / {} / {}", win.to_string().color(Color::Green), draw.to_string().color(Color::Grey0), lose.to_string().color(Color::Red));
}