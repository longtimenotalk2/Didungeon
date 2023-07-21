
use didungeon::game::Game;

fn main() {
    let mut game = Game::new();
    // let mut game = Game::new_solo_auto(114516, 16, 13, 13, 13, 13, 16);
    game.main_loop();

}