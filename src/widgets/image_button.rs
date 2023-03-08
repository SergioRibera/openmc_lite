use egui::{Color32, Image, Rect, Sense, Widget, Vec2, TextureId};

pub struct ImageButton {
    image: Image,
    enabled: bool,
    disable_color: Color32,
    hover_color: Color32,
    click_color: Color32,
    sense: Sense,
}

impl ImageButton {
    pub fn new(texture_id: impl Into<TextureId>, size: impl Into<Vec2>) -> Self {
        Self {
            image: Image::new(texture_id, size),
            enabled: true,
            sense: Sense::click(),
            hover_color: Color32::from_gray(120),
            disable_color: Color32::from_gray(220),
            click_color: Color32::from_gray(220),
        }
    }

    pub fn set_hover_color(mut self, hover_color: Color32) -> Self {
        self.hover_color = hover_color;
        self
    }

    pub fn set_click_color(mut self, click_color: Color32) -> Self {
        self.click_color = click_color;
        self
    }

    pub fn set_disable_color(mut self, color: Color32) -> Self {
        self.disable_color = color;
        self
    }

    /// Select UV range. Default is (0,0) in top-left, (1,1) bottom right.
    pub fn uv(mut self, uv: impl Into<Rect>) -> Self {
        self.image = self.image.uv(uv);
        self
    }

    /// By default, buttons senses clicks.
    /// Change this to a drag-button with `Sense::drag()`.
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    pub fn set_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for ImageButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self {
            image,
            enabled,
            disable_color,
            hover_color,
            click_color,
            sense,
        } = self;

        let size = image.size();
        let (rect, response) = ui.allocate_exact_size(size, sense);

        if ui.is_rect_visible(rect) {
            let image_rect = ui
                .layout()
                .align_size_within_rect(size, rect);

            let color = if !enabled || !ui.is_enabled() {
                disable_color
            } else if response.hovered() {
                hover_color
            } else if response.clicked() {
                click_color
            } else {
                Color32::WHITE
            };

            image.tint(color).paint_at(ui, image_rect);
        }

        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    }
}
