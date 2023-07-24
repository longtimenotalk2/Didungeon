
use didungeon::game::Game;

fn main() {
    // let mut game = Game::new();

    let seed = 0;
    let mut game = Game::new_team(seed);

    game.main_loop();

}