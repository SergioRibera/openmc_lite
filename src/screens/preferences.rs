use eframe::egui::Ui;
use egui_stylist::StylistState;

use crate::settings::LauncherSettings;


pub fn preferences(ui: &mut Ui, theme: &mut StylistState, _conf: &LauncherSettings) {
    theme.ui(ui);
}
