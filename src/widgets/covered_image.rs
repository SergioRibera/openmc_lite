use egui::{Color32, Pos2, Rect};
use egui_extras::RetainedImage;

use super::{calculate_ratio_size, CoveredRatioType};

pub struct CoveredImage;

impl CoveredImage {
    pub fn show(
        ui: &mut egui::Ui,
        image: &RetainedImage,
        container_rect: Rect,
        t: CoveredRatioType,
        tint: Option<Color32>,
    ) {
        if !ui.is_rect_visible(container_rect) {
            return;
        }
        let painter = ui.painter();
        let image_size = image.size_vec2();

        let new_rect = Rect::from_center_size(
            container_rect.center(),
            calculate_ratio_size(image_size, container_rect.size(), t),
        );
        let uv = Rect::from_min_max(Pos2::ZERO, Pos2::new(1., 1.));

        painter.image(
            image.texture_id(ui.ctx()),
            new_rect,
            uv,
            tint.unwrap_or(Color32::WHITE),
        );
    }
}
