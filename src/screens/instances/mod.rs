mod create;

pub use create::*;

use crate::MainApplication;
use eframe::egui::{Grid, RichText, Ui};

pub fn instances(ui: &mut Ui, state: &mut MainApplication) {
    Grid::new("Instances")
        .num_columns(4)
        .striped(true)
        .spacing((10., 10.))
        .min_col_width(300.)
        .min_row_height(300.)
        .show(ui, |ui| {
            let text_size = 20.;
            let add_btn = ui.add(
                eframe::egui::Button::new(RichText::new("Create Instance").size(text_size))
                    .min_size(ui.available_size()),
            );
            if add_btn.clicked() {
                state.create_instance = true;
            }
            state.launcher_config.instances.iter().for_each(|i| {
                ui.label(RichText::new(i.name.clone()).size(text_size));
            });
        });
}