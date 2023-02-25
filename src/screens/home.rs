use eframe::egui::{ComboBox, Layout, Margin, RichText, Ui};
use egui::Button;

use crate::{
    resources::ResourceLoader,
    settings::LauncherSettings,
    widgets::{CoveredImage, CoveredImageType},
};

pub fn home(ui: &mut Ui, conf: &LauncherSettings, res: &ResourceLoader) {
    let mut value = conf
        .last_launched
        .clone()
        .map(|l| l.name)
        .unwrap_or_default();
    eframe::egui::Frame::default()
        .inner_margin(Margin::same(0.))
        .rounding(0.)
        .show(ui, |ui| {
            CoveredImage::show(ui, &res.home_bg, CoveredImageType::Cover, None);
            // ui.image(res.home_bg.texture_id(ui.ctx()), ui.available_size());
            ui.with_layout(Layout::bottom_up(eframe::emath::Align::Min), |ui| {
                ui.add_space(50.);
                ui.horizontal(|ui| {
                    ComboBox::from_label("Select Version")
                        .selected_text(value.as_str())
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.set_min_width(60.0);
                            conf.instances.iter().for_each(|i| {
                                ui.selectable_value(&mut value, i.name.clone(), i.name.clone());
                            })
                        });
                    ui.add_space((ui.available_width() / 2.) - 40.);
                    let _btn_play = ui.add_enabled(
                        !value.is_empty(),
                        Button::new(RichText::new("Jugar").size(32.)),
                    );
                });
            });
        });
}
