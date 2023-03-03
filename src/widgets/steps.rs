use egui::{Color32, FontId, Sense, Stroke, Widget};

#[derive(Clone, Default)]
pub struct Steps {
    current: usize,
    steps: Vec<String>,
}

impl Steps {
    pub fn with_steps(steps: Vec<&str>) -> Self {
        Self { current: 0, steps: steps.iter().map(|s| s.to_string()).collect() }
    }

    pub fn set_steps(&mut self, steps: Vec<&str>) -> &mut Self {
        self.steps = steps.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn set_current(&mut self, curr: usize) -> &mut Self {
        self.current = curr;
        self
    }
}

impl Widget for Steps {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let rect = ui.min_rect();
        let mut pos = rect.center();
        pos.x -= 200.;
        for (i, step) in self.steps.iter().enumerate() {
            let painter = ui.painter();
            if i > self.current {
                painter.circle_stroke(pos, 10., Stroke::new(1.5, Color32::GREEN));
            } else {
                painter.circle_filled(pos, 10., Color32::GREEN);
            }
            {
                pos.x += 25.;
                pos.x += painter
                    .text(
                        pos,
                        egui::Align2::LEFT_CENTER,
                        step,
                        FontId::proportional(24.),
                        Color32::WHITE,
                    )
                    .width()
                    + 10.;
            }
            if i < self.steps.len() - 1 {
                let mut to = pos;
                to.x += 50.;
                painter.line_segment([pos, to], Stroke::new(1.5, Color32::GREEN));
                pos.x += 70.;
            }
        }
        ui.allocate_rect(rect, Sense::click())
    }
}
