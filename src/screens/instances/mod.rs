mod create;
pub mod utils;

use std::{
    cell::RefCell,
    sync::{mpsc::Receiver, Arc, Mutex},
};

pub use create::*;

use eframe::egui::Ui;
use egui::{Button, Color32, Layout, RichText, SidePanel, Vec2};
use log::{debug, info};
use mc_bootstrap::ClientBootstrap;
use mc_downloader::prelude::{ClientDownloader, DownloadVersion};

use crate::{
    data::data_path,
    download_svc::{DownloadProgress, DownloadProgressMessage},
    resources::{icon::Icon, ResourceLoader},
    settings::{LauncherInstance, LauncherSettings},
    widgets::{add_toast, GridWrapped, GridWrappedBuilder, IconButton, ProgressButton},
    MainState,
};

pub struct Instances {
    selected: Option<LauncherInstance>,
    frame_sizes: Vec<Vec2>,
    widget: GridWrapped<LauncherInstance>,
    download_button: ProgressButton,
    resources: ResourceLoader,
    progress: DownloadProgress,
    progress_rcv: Receiver<DownloadProgressMessage>,
}

impl Default for Instances {
    fn default() -> Self {
        let (progress, progress_rcv) = DownloadProgress::new();

        Self {
            progress,
            progress_rcv,
            selected: None,
            frame_sizes: Vec::new(),
            download_button: ProgressButton::default()
                .set_text("Launch")
                .show_progress()
                .set_progress(0.)
                .build(),
            widget: GridWrappedBuilder::default()
                .show_search()
                .set_cell_size((200., 200.))
                .set_button_text("Create Instance")
                .build(),
            resources: ResourceLoader::default(),
        }
    }
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
            let mut grid = self.widget.clone();
            let mut reset = false;
            let mut replaced = false;
            if self.selected.is_some() {
                ui.set_max_width(ui.available_width() - 300.);
            } else {
                ui.set_max_width(ui.available_width());
            }
            grid.set_items(cfg.instances.clone()).show(
                ui,
                Some(|| {
                    widget.reset();
                    reset = true;
                    state.create_instance = true;
                }),
                Some(|_: usize, item: &LauncherInstance, search: &str| {
                    item.name.to_lowercase().contains(&search.to_lowercase())
                }),
                |ui, i, _| {
                    let i = i.clone();
                    let mut frame_sizes = frame_sizes.borrow_mut();
                    ui.horizontal_centered(|ui| {
                        ui.add_space((ui.available_width() - frame_sizes[i].x) / 2.0);
                        let top_space = (ui.available_height() - frame_sizes[i].y) / 2.0;
                        let frame_response = ui.vertical_centered(|ui| {
                            ui.add_space(top_space);
                            let mut icon_path = cfg.instances[i].path.clone();
                            icon_path.push_str("/icon.png");
                            ui.image(
                                Icon::image_from_path(
                                    cfg.instances[i].name.as_str(),
                                    icon_path.as_str(),
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
                |s: usize| {
                    replaced = true;
                    selected.replace(Some(cfg.instances[s].clone()));
                    self.selected = Some(cfg.instances[s].clone());
                },
            );
            let mut select_instance = selected.borrow_mut();
            let mut launch_btn = if reset || replaced {
                self.download_button.set_progress(0.).clone()
            } else {
                self.download_button.clone()
            };
            SidePanel::right(format!("Instance Details - {:?}", selected))
                .exact_width(300.)
                .show_animated_inside(ui, self.selected.is_some(), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        if let Some(mut instance) = select_instance.clone() {
                            if !instance.downloaded {
                                launch_btn.set_text("Download");
                            }
                            let path = instance.path.clone();
                            let mut icon_path = path.clone();
                            icon_path.push_str("/icon.png");
                            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                                ui.add_space(10.);
                                let btn_close = ui.add_sized(
                                    (20., 20.),
                                    IconButton::new(&self.resources.icons.close),
                                );
                                if btn_close.clicked() && !instance.downloading {
                                    info!("Close SidePanel clicked!!");
                                    grid.reset();
                                    *select_instance = None;
                                }
                            });
                            ui.image(
                                Icon::image_from_path(
                                    instance.name.as_str(),
                                    icon_path.as_str(),
                                    egui_extras::image::FitTo::Original,
                                )
                                .unwrap()
                                .id(ui.ctx()),
                                (70., 70.),
                            );
                            ui.add_space(20.);
                            ui.label(RichText::new(instance.name.clone()).size(25.).strong());
                            let v = instance.version.clone().unwrap();
                            if let Some(version) = instance.version.clone() {
                                ui.add_space(10.);
                                ui.label(version.get_version_id());
                            }
                            ui.add_space(20.);
                            // Buttons
                            ui.horizontal(|ui| {
                                if instance.downloading {
                                    while let Ok(msg) = self.progress_rcv.try_recv() {
                                        match msg {
                                            DownloadProgressMessage::Setup(_) => add_toast(
                                                &mut state.toasts,
                                                "Instance",
                                                &format!(
                                                    "Downloading resources for instance: {}",
                                                    instance.name.clone()
                                                ),
                                                crate::widgets::OpenMCToastKind::Info,
                                            ),
                                            DownloadProgressMessage::Update(curr, max) => {
                                                ui.ctx().request_repaint();
                                                launch_btn = self
                                                    .download_button
                                                    .set_progress(curr as f32 / max as f32)
                                                    .set_text("Downloading...")
                                                    .build();
                                            }
                                            DownloadProgressMessage::End => {
                                                debug!("Message Downloaded!!");
                                                instance.downloading = false;
                                                instance.downloaded = true;
                                                add_toast(
                                                    &mut state.toasts,
                                                    "Instance",
                                                    &format!(
                                                        "Resources for instance are Downloaded: {}",
                                                        instance.name.clone()
                                                    ),
                                                    crate::widgets::OpenMCToastKind::Info,
                                                );
                                            }
                                        }
                                    }
                                }
                                // Launch
                                let width = ui.available_width() - 10.;
                                ui.add_enabled_ui(!instance.downloading, |ui| {
                                    let btn =
                                        ui.add_sized(Vec2::new(width, 50.), launch_btn.clone());
                                    if btn.clicked() {
                                        if !instance.downloaded && !instance.downloading {
                                            let progress = self.progress.clone();
                                            std::thread::spawn(move || {
                                                debug!("creating thread and start download");
                                                ClientDownloader::new()
                                                    .unwrap()
                                                    .download_version(
                                                        &v.get_version_id(),
                                                        data_path("").to_str().unwrap(),
                                                        Some(Arc::new(Mutex::new(progress))),
                                                    )
                                                    .unwrap();
                                                debug!("Downloaded");
                                            });
                                            debug!("Downloading");
                                            instance.downloading = true;
                                            self.download_button.set_text("Downloading...");
                                        }
                                        if instance.downloaded {
                                            let v = instance.version.clone().unwrap();
                                            ClientBootstrap::new(
                                                "null",
                                                data_path("").to_str().unwrap(),
                                                "/usr/lib/jvm/java-8-openjdk/bin/java",
                                                "SergioRibera",
                                                "null",
                                                &v.get_version_id(),
                                                &v.get_version_type(),
                                            )
                                            .launch()
                                            .unwrap();
                                        }
                                    }
                                });
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
                                let open_btn = ui.add_sized(
                                    Vec2::new(width, 30.),
                                    Button::new("Open").wrap(true),
                                );
                                if open_btn.clicked() {
                                    open::that(path).unwrap();
                                }
                                ui.add_enabled_ui(!instance.downloading, |ui| {
                                    let delete_btn = ui.add_sized(
                                        Vec2::new(width, 30.),
                                        Button::new("Delete").wrap(true).fill(Color32::LIGHT_RED),
                                    );
                                    if delete_btn.clicked() {
                                        *select_instance = None;
                                        cfg.remove_instance(instance.name.clone());
                                    }
                                });
                            });
                            ui.add_space(20.);
                            if select_instance.is_some() {
                                select_instance.replace(instance.clone());
                            }
                        }
                    });
                });
            self.selected = select_instance.clone();
            self.frame_sizes = frame_sizes.take();
            self.download_button = launch_btn;
            if reset {
                grid.reset();
            }
            self.widget = grid.clone();
        });
    }
}
