use egui::{Color32, RichText, Sense, Stroke, Ui, Vec2};

#[derive(Default)]
pub struct GridWrapped {
    selected: Option<usize>,
}

impl GridWrapped {
    #[allow(clippy::too_many_arguments)]
    pub fn show(
        &mut self,
        ui: &mut Ui,
        btn_str: &str,
        cell_size: impl Into<Vec2> + Clone,
        items: usize,
        draw_item: impl FnOnce(&mut Ui, usize) + Copy,
        on_btn_click: impl FnOnce() + Copy,
        on_change: impl FnOnce(usize) + Copy,
    ) {
        egui::ScrollArea::vertical()
            .min_scrolled_width(ui.available_width())
            .min_scrolled_height(ui.available_height())
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let (rect, _resp) =
                        ui.allocate_at_least(cell_size.clone().into(), Sense::click());

                    ui.allocate_ui_at_rect(rect, |ui| {
                        let btn = ui.add(
                            eframe::egui::Button::new(RichText::new(btn_str).size(20.))
                                .wrap(true)
                                .min_size(egui::Vec2::new(70., 70.)),
                        );
                        if btn.clicked() {
                            on_btn_click();
                        }
                        ui.add_space(5.);
                    });

                    for i in 0..items {
                        let (rect, resp) =
                            ui.allocate_at_least(cell_size.clone().into(), Sense::click());
                        let mut rect_margin = rect;
                        rect_margin.max.x += 5.;
                        rect_margin.max.y += 5.;

                        ui.allocate_ui_at_rect(rect_margin, |ui| {
                            let color = if let Some(selected) = self.selected {
                                if selected == i {
                                    Color32::from_gray(64)
                                } else {
                                    Color32::TRANSPARENT
                                }
                            } else {
                                Color32::TRANSPARENT
                            };
                            if resp.hovered() {
                                ui.painter().rect_stroke(
                                    rect,
                                    5.,
                                    Stroke::new(2., Color32::from_gray(64)),
                                );
                            } else {
                                ui.painter().rect_filled(rect, 5., color);
                            }
                            draw_item(ui, i);
                        });
                        if resp.clicked() {
                            self.selected = Some(i);
                            on_change(i);
                        }
                    }
                });
            });
    }
}
