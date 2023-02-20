use egui::{Color32, FontId, Layout, RichText, Stroke};
use egui_extras::Size;

use crate::{
    settings::{LauncherInstance, MinecraftVersion},
    MainApplication,
};

type StepCallback = fn(&mut CreateInstance, &mut egui::Ui);

static STEPS: &[(&str, StepCallback)] = &[
    ("Name", set_name),
    ("Icon", set_icon),
    ("Version", set_version),
];

#[derive(Clone)]
pub struct CreateInstance {
    curr_step: u8,
    max_step: u8,
    name: String,
    path: String,
    icon_path: String,
    version: Option<MinecraftVersion>,
}

impl CreateInstance {
    pub fn new() -> Self {
        Self {
            curr_step: 0,
            max_step: STEPS.len() as u8 - 1,
            name: String::new(),
            path: String::new(),
            icon_path: String::new(),
            version: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut MainApplication) {
        ui.add_space(20.);
        egui_extras::StripBuilder::new(ui)
            .size(Size::relative(0.1)) // Progress
            .size(Size::relative(0.8)) // From
            .size(Size::relative(0.1)) // Buttons
            .vertical(|mut strip| {
                strip.strip(|strip_builder| {
                    strip_builder
                        .size(Size::relative(0.25)) // Margin
                        .size(Size::relative(0.5)) // Content
                        .size(Size::relative(0.25)) // Margin
                        .horizontal(|mut strip| {
                            strip.empty();
                            strip.cell(|ui| {
                                ui.vertical_centered_justified(|ui| {
                                    let rect = ui.min_rect();
                                    let mut pos = rect.center();
                                    pos.x -= 200.;
                                    for (i, (step, _)) in STEPS.iter().enumerate() {
                                        let i = i as u8;
                                        let painter = ui.painter();
                                        if i > self.curr_step {
                                            painter.circle_stroke(
                                                pos,
                                                10.,
                                                Stroke::new(1.5, Color32::GREEN),
                                            );
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
                                        if i < self.max_step {
                                            let mut to = pos.clone();
                                            to.x += 50.;
                                            painter.line_segment(
                                                [pos, to],
                                                Stroke::new(1.5, Color32::GREEN),
                                            );
                                            pos.x += 70.;
                                        }
                                    }
                                });
                            });
                            strip.empty();
                        });
                });
                strip.cell(|ui| {
                    let i = self.curr_step.clone() as usize;
                    STEPS[i].1(self, ui);
                });
                strip.cell(|ui| {
                    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                        if self.curr_step == self.max_step {
                            if ui.button("Finish").clicked() {
                                state.sub_title = String::new();
                                state.create_instance = false;
                            }
                        } else {
                            if ui.button("Next").clicked() {
                                self.curr_step += 1;
                            }
                        }
                        if self.curr_step > 0 {
                            if ui.button("Prev").clicked() {
                                self.curr_step -= 1;
                            }
                        }
                    });
                });
            });
    }
}

fn create_label(ui: &mut egui::Ui, content: &str) {
    ui.label(RichText::new(content).fallback_text_style(egui::TextStyle::Body));
    ui.add_space(10.);
}

fn set_name(data: &mut CreateInstance, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(30.);
        create_label(ui, "What Name have for your instance?");
        ui.add(egui::TextEdit::singleline(&mut data.name).hint_text("My Best Minecraft Instance"));
    });
}

fn set_icon(_data: &mut CreateInstance, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        // Icon
        create_label(ui, "Choose an icon that characterizes your instance");
        ui.add_space(20.);
        ui.horizontal_centered(|ui| {
            egui::ScrollArea::vertical()
                .min_scrolled_width(ui.available_width())
                .min_scrolled_height(ui.available_height())
                .show(ui, |ui| {
                    egui::Grid::new("icons")
                        .num_columns(4)
                        .striped(true)
                        .spacing((10., 10.))
                        .min_col_width(100.)
                        .min_row_height(100.)
                        .show(ui, |ui| {
                            let _add_btn = ui.add(
                                eframe::egui::Button::new(RichText::new("Custom").size(20.))
                                    .wrap(true)
                                    .min_size(ui.available_size()),
                            );
                        });
                });
        });
    });
}

fn set_version(_data: &mut CreateInstance, _ui: &mut egui::Ui) {}
