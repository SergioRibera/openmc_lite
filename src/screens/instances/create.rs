use egui::{Color32, FontId, Layout, RichText, Sense, Stroke, Ui};
use egui_extras::Size;
use log::info;

use crate::{
    data::config_path,
    resources::icon::Icon,
    settings::{LauncherInstance, MinecraftVersion},
    widgets::{GridWrapped, IconButton},
    MainApplication, MainState,
};

type StepCallback = fn(&mut CreateInstance, &mut egui::Ui);

static STEPS: &[(&str, StepCallback)] = &[
    ("Name", set_name),
    ("Icon", set_icon),
    ("Version", set_version),
];

pub struct CreateInstance {
    curr_step: u8,
    max_step: u8,
    name: String,
    icons: Vec<Icon>,
    grid: GridWrapped,
    path: String,
    icon_selected: usize,
    version: Option<MinecraftVersion>,
}

impl CreateInstance {
    pub fn new() -> Self {
        let path_icons = config_path("icons");

        let icons = path_icons
            .read_dir()
            .unwrap()
            .flat_map(|f| f)
            .filter(|f| f.file_name().to_str().unwrap().ends_with(".png"))
            .flat_map(|f| {
                Icon::image_from_path(
                    f.file_name().to_str().unwrap(),
                    f.path().to_str().unwrap(),
                    egui_extras::image::FitTo::Size(80, 80),
                )
            })
            .collect();

        Self {
            icons,
            curr_step: 0,
            max_step: STEPS.len() as u8 - 1,
            grid: GridWrapped::default(),
            name: String::new(),
            path: String::new(),
            icon_selected: 0,
            version: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut MainState) {
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

fn set_icon(data: &mut CreateInstance, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        // Icon
        create_label(ui, "Choose an icon that characterizes your instance");
        ui.add_space(20.);
        data.grid.show(
            ui,
            "Other",
            (100., 100.),
            data.icons.len(),
            |ui, i| {
                ui.centered_and_justified(|ui| {
                    ui.image(data.icons[i].id(ui.ctx()), (50., 50.));
                });
            },
            || {
                info!("Other clicked");
            },
            |selected: usize| {
                info!("Icon '{selected}' is clicked");
            },
        )
    });
}

fn set_version(_data: &mut CreateInstance, _ui: &mut egui::Ui) {}
