
// use didungeon::game::Game;

// fn main() {
//     let mut game = Game::new();

//     // let seed = 0;
//     // let mut game = Game::new_team(seed);

//     game.main_loop();

// }

use didungeon::gui::DidungeonApp;


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Didungeon", native_options, Box::new(|cc| {
        Box::new(DidungeonApp::new(cc))
    })).unwrap();
}



