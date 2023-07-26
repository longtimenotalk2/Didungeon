
// use didungeon::game::Game;

// fn main() {
//     let mut game = Game::new();

//     // let seed = 0;
//     // let mut game = Game::new_team(seed);

//     game.main_loop();

// }

use colorful::core::StrMarker;
use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Didungeon", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}




impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            name: "Elis".to_string(),
            age: 20,
        }
    }
}

#[derive(Default)]
struct MyEguiApp {
    name : String,
    age : i32,
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let name = &mut self.name;
            let age = &mut self.age;
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(name);
            });
            ui.add(egui::Slider::new(age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                *age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });

        
    }
}