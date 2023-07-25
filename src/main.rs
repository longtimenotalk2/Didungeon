
// use didungeon::game::Game;

// fn main() {
//     // let mut game = Game::new();

//     let seed = 0;
//     let mut game = Game::new_team(seed);

//     game.main_loop();

// }

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.label("This is a label");
            ui.label("This is a label");
            ui.hyperlink("https://github.com/emilk/egui");
            let mut my_string = String::new();
            ui.text_edit_singleline(&mut my_string);
            if ui.button("Click me").clicked() { }
            let mut my_f32 = 0.0;
            ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0));
            ui.add(egui::DragValue::new(&mut my_f32));

            let mut my_boolean = false;
            ui.checkbox(&mut my_boolean, "Checkbox");

            #[derive(PartialEq)]
            enum Enum { First, Second, Third }
            let mut my_enum = Enum::First;
            ui.horizontal(|ui| {
                ui.radio_value(&mut my_enum, Enum::First, "First");
                ui.radio_value(&mut my_enum, Enum::Second, "Second");
                ui.radio_value(&mut my_enum, Enum::Third, "Third");
            });

            ui.separator();

            // let my_image = "assets/graphic/city.png";
            // ui.image(my_image, [640.0, 480.0]);

            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.label("Not much, as it turns out");
            });
        });
    }
}