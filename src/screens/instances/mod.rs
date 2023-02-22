mod create;
pub mod utils;

use std::cell::RefCell;

pub use create::*;

use eframe::egui::Ui;
use egui::{Button, Color32, Layout, RichText, SidePanel, Vec2};

use crate::{
    resources::{icon::Icon, ResourceLoader},
    settings::{LauncherInstance, LauncherSettings},
    widgets::GridWrapped,
    MainState,
};

#[derive(Default)]
pub struct Instances {
    selected: Option<LauncherInstance>,
    widget: GridWrapped,
    resources: ResourceLoader,
}

impl Instances {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        cfg: &LauncherSettings,
        widget: &mut CreateInstance,
        state: &mut MainState,
    ) {
        let selected = RefCell::new(self.selected.clone());
        ui.add_space(20.);
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            self.widget.show(
                ui,
                Some("Create Instance"),
                (200., 200.),
                cfg.instances.len(),
                |ui, i| {
                    ui.centered_and_justified(|ui| {
                        // ui.group(|ui| {
                        // ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                        ui.image(
                            Icon::image_from_path(
                                cfg.instances[i].name.as_str(),
                                cfg.instances[i].icon_path.as_str(),
                                egui_extras::image::FitTo::Original,
                            )
                            .unwrap()
                            .id(ui.ctx()),
                            (50., 50.),
                        );
                        // });
                        // ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                        // ui.label(RichText::new(cfg.instances[i].name.clone()).size(20.));
                        // });
                        // });
                    });
                },
                || {
                    widget.reset();
                    state.create_instance = true;
                },
                |s: usize| {
                    selected.replace(Some(cfg.instances[s].clone()));
                    self.selected = Some(cfg.instances[s].clone());
                },
            );
            let selected = selected.take();
            SidePanel::right(format!("Instance Details - {:?}", selected))
                .exact_width(300.)
                .show_animated_inside(ui, selected.is_some(), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        if let Some(instance) = selected {
                            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                                ui.image(self.resources.icons.close.id(ui.ctx()), (20., 20.));
                            });
                            ui.image(
                                Icon::image_from_path(
                                    instance.name.as_str(),
                                    instance.icon_path.as_str(),
                                    egui_extras::image::FitTo::Original,
                                )
                                .unwrap()
                                .id(ui.ctx()),
                                (70., 70.),
                            );
                            ui.add_space(20.);
                            ui.label(RichText::new(instance.name.clone()).size(25.).strong());
                            ui.add_space(20.);
                            // Buttons
                            ui.horizontal(|ui| {
                                let width = ui.available_width() - 10.;
                                ui.add_sized(
                                    Vec2::new(width, 50.),
                                    Button::new("Launch").wrap(true).fill(Color32::LIGHT_GREEN),
                                );
                            });
                            ui.add_space(10.);
                            ui.horizontal(|ui| {
                                let width = ui.available_width() / 3. - 10.;

                                ui.add_sized(Vec2::new(width, 30.), Button::new("Edit").wrap(true));
                                ui.add_sized(Vec2::new(width, 30.), Button::new("Open").wrap(true));
                                ui.add_sized(
                                    Vec2::new(width, 30.),
                                    Button::new("Delete").wrap(true).fill(Color32::LIGHT_RED),
                                );
                            });
                            ui.add_space(20.);
                            // Information
                            if let Some(version) = instance.version {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Version:").size(15.).strong());
                                    ui.label(version.to_string());
                                });
                            }
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Java:").size(15.).strong());
                                ui.label("19");
                            });
                        }
                    });
                });
        });
    }
}
