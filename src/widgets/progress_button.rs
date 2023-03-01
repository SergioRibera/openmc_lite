use egui::{Color32, CursorIcon, FontId, Rect, Sense, Widget};

#[derive(Clone)]
pub struct ProgressButton {
    progress: f32,
    text: String,
    text_color: Color32,
    text_size: f32,
    padding: f32,
    background_color: Color32,
    fill_color: Color32,
    show_progress: bool,
    border_round: f32,
}

impl Default for ProgressButton {
    fn default() -> Self {
        Self {
            progress: 0.,
            text_color: Color32::WHITE,
            background_color: Color32::BLUE,
            fill_color: Color32::LIGHT_BLUE,
            show_progress: false,
            text: String::new(),
            text_size: 15.,
            padding: 3.,
            border_round: 3.,
        }
    }
}

impl ProgressButton {
    pub fn set_progress(&mut self, progress: f32) -> &mut Self {
        self.progress = progress;
        self
    }

    pub fn calcule_progress(&mut self, curr: u64, max: u64) -> &mut Self {
        self.progress = curr as f32 / max as f32;
        self
    }

    pub fn set_text_color(&mut self, text_color: Color32) -> &mut Self {
        self.text_color = text_color;
        self
    }

    pub fn set_fill_color(&mut self, fill_color: Color32) -> &mut Self {
        self.fill_color = fill_color;
        self
    }

    pub fn show_progress(&mut self) -> &mut Self {
        self.show_progress = true;
        self
    }

    pub fn set_text_size(&mut self, text_size: f32) -> &mut Self {
        self.text_size = text_size;
        self
    }

    pub fn set_padding(&mut self, padding: f32) -> &mut Self {
        self.padding = padding;
        self
    }

    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self
    }

    pub fn set_border_round(&mut self, border_round: f32) -> &mut Self {
        self.border_round = border_round;
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl Widget for ProgressButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let rect = ui.max_rect();

        let mut pos = rect.left_center();
        pos.y = rect.min.y;
        let max_width = ui.available_width();
        let mut to = pos;
        to.x += self.progress * max_width;
        to.y = rect.max.y;

        let fill_rect = Rect::from_min_max(pos, to);
        // fill_rect.max.y += self.text_size + (self.padding * 2.);

        let resp = ui.allocate_rect(rect, Sense::click());
        let painter = ui.painter();
        let mut text_pos = rect.center();
        text_pos.x -= (self.text.len() / 2 * 7) as f32;

        painter.rect_filled(rect, self.border_round, self.background_color);
        if self.show_progress {
            painter.rect_filled(fill_rect, self.border_round, self.fill_color);
        }
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            self.text.clone(),
            FontId::monospace(self.text_size),
            self.text_color,
        );
        resp.on_hover_cursor(CursorIcon::PointingHand)
    }
}
