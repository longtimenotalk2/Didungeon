use egui::{Ui, RichText, Color32};

use crate::game::unit::Unit;

pub fn show_unit(ui : &mut Ui, unit : &Unit) {

    egui::Frame::group(ui.style())
    .fill(egui::Color32::LIGHT_GRAY)
    .show(ui, |ui| {
        ui.set_height(200.0);
        ui.set_width(123.6);
        // 第一行，显示：速/移/伤。
        ui.horizontal_top(|ui| {
            //拆分3个vertical
            ui.vertical(|ui| {
                ui.colored_label(Color32::DARK_GRAY, format!("速:{:>2}", unit.spd()));
            });
            ui.vertical_centered(|ui| {
                ui.label(format!("移:{:>2}", unit.move_range()));
            });
            ui.vertical(|ui| {
                ui.add(egui::Label::new(
                    RichText::new(format!("伤:{:>2}", unit.get_inj())).color(Color32::RED)
                ).wrap(false));
            });
        });
        ui.separator();
        ui.horizontal_centered(|ui| {
            ui.add(egui::Label::new(
                RichText::new("安捷列姆").size(20.0)
                )
            )
            // ui.vertical_centered_justified(|ui| {
            //     ui.add(egui::Label::new(
            //         RichText::new("安捷列姆").size(20.0)
            //         )
            //     )
            // });
        });
        ui.separator();
        // 最后一行，显示：；力/技/速。
        ui.horizontal_top(|ui| {
            //拆分3个vertical
            ui.vertical(|ui| {
                ui.colored_label(Color32::DARK_GRAY, format!("力:{:>2}", unit.str()));
            });
            ui.centered_and_justified(|ui| {
                ui.label(format!("技:{:>2}", unit.dex()));
            });
            ui.add(egui::Label::new(
                RichText::new(format!("敏:{:>2}", unit.agi())).color(Color32::RED)
            ).wrap(false));
            // ui.vertical(|ui| {
            //     ui.add(egui::Label::new(
            //         RichText::new(format!("敏:{:>2}", unit.agi())).color(Color32::RED)
            //     ).wrap(false));
            // });
        });
    });
    

}