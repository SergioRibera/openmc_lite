mod create;
pub mod utils;

use std::cell::RefCell;

pub use create::*;

use eframe::egui::Ui;
use egui::{Button, Color32, Layout, RichText, SidePanel, Vec2};
use log::info;

use crate::{
    resources::{icon::Icon, ResourceLoader},
    settings::{LauncherInstance, LauncherSettings},
    widgets::{GridWrapped, IconButton},
    MainState,
};

#[derive(Default)]
pub struct Instances {
    selected: Option<LauncherInstance>,
    frame_sizes: Vec<Vec2>,
    widget: GridWrapped,
    resources: ResourceLoader,
}

impl Instances {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        cfg: &mut LauncherSettings,
        widget: &mut CreateInstance,
        state: &mut MainState,
    ) {
        if self.frame_sizes.len() != cfg.instances.len() {
            self.frame_sizes
                .resize(cfg.instances.len(), Vec2::default());
        }
        let selected = RefCell::new(self.selected.clone());
        let frame_sizes = RefCell::new(self.frame_sizes.clone());
        ui.add_space(20.);
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            self.widget.show(
                ui,
                Some("Create Instance"),
                (200., 200.),
                cfg.instances.len(),
                |ui, i| {
                    let mut frame_sizes = frame_sizes.borrow_mut();
                    ui.horizontal_centered(|ui| {
                        ui.add_space((ui.available_width() - frame_sizes[i].x) / 2.0);
                        let top_space = (ui.available_height() - frame_sizes[i].y) / 2.0;
                        let frame_response = ui.vertical_centered(|ui| {
                            ui.add_space(top_space);
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
                            ui.add_space(20.);
                            ui.label(RichText::new(cfg.instances[i].name.clone()).size(20.));
                        });
                        frame_sizes[i] = frame_response.response.rect.size();
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
            let mut select_instance = selected.borrow_mut();
            SidePanel::right(format!("Instance Details - {:?}", selected))
                .exact_width(300.)
                .show_animated_inside(ui, self.selected.is_some(), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        if let Some(instance) = select_instance.clone() {
                            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                                let btn_close = ui.add_sized(
                                    (20., 20.),
                                    IconButton::new(&self.resources.icons.close),
                                );
                                if btn_close.clicked() {
                                    info!("Close SidePanel clicked!!");
                                    *select_instance = None;
                                }
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
                            if let Some(version) = instance.version {
                                ui.add_space(10.);
                                ui.label(version.to_string());
                            }
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

                                ui.add_enabled_ui(false, |ui| {
                                    ui.add_sized(
                                        Vec2::new(width, 30.),
                                        Button::new("Edit").wrap(true),
                                    );
                                });
                                ui.add_sized(Vec2::new(width, 30.), Button::new("Open").wrap(true));
                                let delete_btn = ui.add_sized(
                                    Vec2::new(width, 30.),
                                    Button::new("Delete").wrap(true).fill(Color32::LIGHT_RED),
                                );
                                if delete_btn.clicked() {
                                    *select_instance = None;
                                    cfg.remove_instance(instance.name.clone());
                                }
                            });
                            ui.add_space(20.);
                        }
                    });
                });
            self.selected = select_instance.clone();
            self.frame_sizes = frame_sizes.take();
        });
    }
}
