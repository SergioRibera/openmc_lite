use eframe::egui::{ComboBox, Layout, Ui};
use egui::{Color32, ImageButton, Sense, Stroke};

use crate::{
    resources::ResourceLoader,
    settings::LauncherSettings,
    widgets::{CoveredImage, CoveredImageType},
};

#[inline]
pub fn home(ui: &mut Ui, conf: &LauncherSettings, res: &ResourceLoader) {
    let mut value = conf
        .last_launched
        .clone()
        .map(|l| l.name)
        .unwrap_or_default();
    ui.with_layout(Layout::bottom_up(eframe::emath::Align::Max), |ui| {
        let max_rect = ui.max_rect();
        ui.set_clip_rect(max_rect);
        CoveredImage::show(ui, &res.home_bg, max_rect, CoveredImageType::Cover, None);
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
        ui.with_layout(
            Layout::centered_and_justified(egui::Direction::LeftToRight),
            |ui| {
                let style = ui.style_mut();
                style.visuals.button_frame = false;
                style.visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;
                style.visuals.widgets.active.bg_stroke = Stroke::NONE;
                style.visuals.widgets.open.weak_bg_fill = Color32::TRANSPARENT;
                style.visuals.widgets.open.bg_stroke = Stroke::NONE;
                style.visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
                style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
                style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
                style.visuals.widgets.noninteractive.weak_bg_fill = Color32::TRANSPARENT;
                style.visuals.widgets.noninteractive.bg_stroke = Stroke::NONE;
                let _btn_play = ui
                    .add_enabled(
                        !value.is_empty(),
                        ImageButton::new(res.btn_bg.texture_id(ui.ctx()), (250., 80.))
                            .sense(Sense::click()),
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
            },
        );
    });
}
