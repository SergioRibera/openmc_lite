use egui::{Color32, Pos2, Rect, Vec2};
use egui_extras::RetainedImage;

pub struct CoveredImage;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CoveredImageType {
    Cover,
    Container,
}

impl CoveredImage {
    pub fn show(
        ui: &mut egui::Ui,
        image: &RetainedImage,
        t: CoveredImageType,
        tint: Option<Color32>,
    ) {
        let painter = ui.painter();
        let container_rect = ui.max_rect();
        let image_size = image.size_vec2();
        let w_ratio = container_rect.width() / image_size.x;
        let h_ratio = container_rect.height() / image_size.y;

        let ratio = if t == CoveredImageType::Cover {
            w_ratio.max(h_ratio)
        } else {
            w_ratio.min(h_ratio)
        };

        let new_rect = Rect::from_center_size(
            container_rect.center(),
            Vec2::new(image_size.x * ratio, image_size.y * ratio),
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
