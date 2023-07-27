use crate::game::unit::Unit;

pub mod fonts;
mod show;

pub struct DidungeonApp {
    unit : Unit
}

impl DidungeonApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::setup_custom_fonts(&cc.egui_ctx);
        Self::init()
    }
}

impl DidungeonApp {
    fn init() -> Self {
        let mut unit = Unit::new_noal(0, 0);
        unit.take_dmg(14);
        Self {
            unit,
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