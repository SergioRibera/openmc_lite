use eframe::egui::{ComboBox, Layout, Ui};
use log::info;

use crate::{
    resources::ResourceLoader,
    settings::LauncherSettings,
    widgets::{CoveredImage, CoveredRatioType, ImageButton},
};

#[inline]
pub fn home(ui: &mut Ui, conf: &LauncherSettings, res: &ResourceLoader) {
    // if !ui.is_rect_visible(ui.max_rect()) {
    //     return;
    // }
    let mut value = conf
        .last_launched
        .clone()
        .map(|l| l.name)
        .unwrap_or_default();
    ui.with_layout(Layout::bottom_up(eframe::emath::Align::Center), |ui| {
        let max_rect = ui.max_rect();
        ui.set_clip_rect(max_rect);
        CoveredImage::show(ui, &res.home_bg, max_rect, CoveredRatioType::Cover, None);
        ui.add_space(10.);
        ui.with_layout(Layout::right_to_left(eframe::emath::Align::Max), |ui| {
            ui.add_space(10.);
            ComboBox::from_id_source("Select Version")
                .selected_text(value.as_str())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    conf.instances.iter().for_each(|i| {
                        ui.selectable_value(&mut value, i.name.clone(), i.name.clone());
                    })
                });
        });
        let btn_play = ui.add_enabled(
            !value.is_empty(),
            ImageButton::new(res.btn_bg.texture_id(ui.ctx()), (250., 80.)),
        );
        if btn_play.clicked() {
            info!("Btn Clicked");
        }
    });
}
