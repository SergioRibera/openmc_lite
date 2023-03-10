use eframe::egui::Ui;
use egui_stylist::StylistState;
use log::trace;

use crate::{
    data::theme::{load_theme, save_theme},
    settings::LauncherSettings,
};

pub fn preferences(ui: &mut Ui, theme: &mut StylistState, conf: &mut LauncherSettings) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            if ui.button("Load Theme").clicked() {
                trace!("Before load theme");
                load_theme(theme, conf, ui.ctx());
            }
            ui.add_space(10.);
            if ui.button("Save Current").clicked() {
                trace!("Before save theme");
                save_theme(theme, conf, ui.ctx());
            }
        });
        ui.add_space(10.);
        theme.ui(ui);
    });
}
