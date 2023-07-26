
// use didungeon::game::Game;

// fn main() {
//     let mut game = Game::new();

//     // let seed = 0;
//     // let mut game = Game::new_team(seed);

//     game.main_loop();

// }

// use eframe::egui;
// use egui_extras::RetainedImage;

// fn main() {
//     let native_options = eframe::NativeOptions::default();
//     eframe::run_native("Didungeon", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
// }




// impl MyEguiApp {
//     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
//         // Restore app state using cc.storage (requires the "persistence" feature).
//         // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
//         // for e.g. egui::PaintCallback.
//         let path = "Elis.png";
//         Self::default()
//     }
// }

// struct MyEguiApp {
//     name : String,
//     age : i32,
//     image : RetainedImage,
// }

// impl Default for MyEguiApp {
//     fn default() -> Self {
//         Self {
//             name : "Elis".to_string(),
//             age : 20,
//             image: RetainedImage::from_image_bytes(
//                 "../assets/graphic/Elis.png",
//                 include_bytes!("../assets/graphic/Elis.png"),
//             )
//             .unwrap(),
//         }
//     }
// }

// impl eframe::App for MyEguiApp {
//    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             let name = &mut self.name;
//             let age = &mut self.age;
//             ui.heading("My egui Application");
//             ui.horizontal(|ui| {
//                 ui.label("Your name: ");
//                 ui.text_edit_singleline(name);
//             });
//             ui.add(egui::Slider::new(age, 0..=120).text("age"));
//             if ui.button("Click each year").clicked() {
//                 *age += 1;
//             }
//             ui.label(format!("Hello '{name}', age {age}"));
//         });

        
//     }
// }


#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_extras::RetainedImage;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 900.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    image: RetainedImage,
    tint: egui::Color32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image: RetainedImage::from_image_bytes(
                "rust-logo-256x256.png",
                include_bytes!("rust-logo-256x256.png"),
            )
            .unwrap(),
            tint: egui::Color32::from_rgb(255, 0, 255),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is an image:");
            self.image.show(ui);

            ui.heading("This is a rotated image with a tint:");
            ui.add(
                egui::Image::new(self.image.texture_id(ctx), self.image.size_vec2())
                    .rotate(45.0_f32.to_radians(), egui::Vec2::splat(0.5))
                    .tint(self.tint),
            );

            ui.horizontal(|ui| {
                ui.label("Tint:");
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut self.tint,
                    egui::color_picker::Alpha::BlendOrAdditive,
                );
            });

            ui.heading("This is an image you can click:");
            ui.add(egui::ImageButton::new(
                self.image.texture_id(ctx),
                self.image.size_vec2(),
            ));
        });
    }
}