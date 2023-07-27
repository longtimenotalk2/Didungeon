use crate::game::unit::Unit;


mod show;

pub struct DidungeonApp {
    unit : Unit
}

impl DidungeonApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::init()
    }
}

impl DidungeonApp {
    fn init() -> Self {
        Self {
            unit: Unit::new_noal(0, 0),
        }
    }
}

impl eframe::App for DidungeonApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            show::show_unit(ui, &self.unit);
        });
    }
}