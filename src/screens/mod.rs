use eframe::egui::{RichText, Ui};

mod account;
mod home;
mod instances;
mod preferences;

pub use self::preferences::*;
pub use account::*;
pub use home::*;
pub use instances::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewType {
    Home,
    Instances,
    Preferences,
}

pub fn tab_buttons(ui: &mut Ui, view: &mut ViewType) {
    ui.horizontal(|ui| {
        let style = ui.style_mut();
        style.visuals.button_frame = false;

        ui.selectable_value(view, ViewType::Home, RichText::new("Play").size(32.));
        ui.selectable_value(
            view,
            ViewType::Instances,
            RichText::new("Instances").size(32.),
        );
        ui.selectable_value(
            view,
            ViewType::Preferences,
            RichText::new("Preferences").size(32.),
        );
    });
}
