
use didungeon::game::Game;

fn main() {
    // let mut game = Game::new();
    let str1 = 10;
    let dex1 = 10;
    let agi1 = 10;
    let str2 = 10;
    let dex2 = 10;
    let agi2 = 10;

    let seed = 0;

    let mut game = Game::new_solo_auto(seed, str1, dex1, agi1, str2, dex2, agi2);
    game.main_loop();

}