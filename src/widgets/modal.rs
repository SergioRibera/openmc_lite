use eframe::epaint::Shadow;
use egui::{Align2, Area, Color32, Frame, Id, Key, Layout, Margin, Order, Stroke, Ui, Vec2};

pub struct ModalBuilder {
    id: String,
    layout: Layout,
    size: Option<Vec2>,
}

#[derive(Clone)]
pub struct Modal {
    id: Id,
    layout: Layout,
    size: Option<Vec2>,
}

impl Default for ModalBuilder {
    fn default() -> Self {
        Self {
            id: "__modal".to_string(),
            layout: Layout::centered_and_justified(egui::Direction::LeftToRight),
            size: None,
        }
    }
}

impl ModalBuilder {
    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = id.to_string();
        self
    }

    pub fn set_layout(&mut self, layout: Layout) -> &mut Self {
        self.layout = layout;
        self
    }

    /// Values from 0.0 to 1.0
    pub fn set_size_percent(&mut self, width: impl Into<Vec2>) -> &mut Self {
        self.size = Some(width.into());
        self
    }

    pub fn build(&self) -> Modal {
        Modal {
            id: Id::new(self.id.clone()),
            layout: self.layout,
            size: self.size,
        }
    }
}

impl Modal {
    pub fn open_modal(&self, ui: &mut Ui) {
        ui.memory_mut(|mem| mem.open_popup(self.id));
        log::info!("Memory Saved!");
    }

    pub fn show(&self, ui: &mut Ui, mut content: impl FnMut(&mut Ui)) {
        if ui.memory(|mem| mem.is_popup_open(self.id)) {
            let rect = ui.max_rect();

            Area::new(self.id)
                .order(Order::Foreground)
                .constrain(true)
                .fixed_pos(rect.center())
                .pivot(Align2::CENTER_CENTER)
                .interactable(true)
                .show(ui.ctx(), |ui| {
                    // Draw BG
                    Frame::none()
                        .fill(Color32::from_black_alpha(120))
                        .shadow(Shadow::NONE)
                        .stroke(Stroke::NONE)
                        .inner_margin(Margin::same(0.))
                        .outer_margin(Margin::same(0.))
                        .show(ui, |ui| {
                            // Draw Content
                            ui.set_min_size(rect.size());
                            ui.with_layout(self.layout, |ui| {
                                let frame = Frame::popup(ui.style()).shadow(Shadow::NONE);
                                frame.show(ui, |ui| {
                                    if let Some(size) = self.size {
                                        let rect_size = rect.size();
                                        ui.set_width(rect_size.x * size.x);
                                        ui.set_height(rect_size.y * size.y);
                                    }
                                    content(ui);
                                });
                            });
                        });
                });

            if ui.input(|i| i.key_pressed(Key::Escape)) {
                ui.memory_mut(|mem| mem.close_popup());
            }
        }
    }
}
