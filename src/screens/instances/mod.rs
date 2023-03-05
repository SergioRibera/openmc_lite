mod create;
pub mod utils;

use std::{
    cell::RefCell,
    sync::{mpsc::Receiver, Arc, Mutex},
};

pub use create::*;

use eframe::egui::Ui;
use egui::{Button, Color32, Layout, RichText, SidePanel, Vec2};
use egui_extras::image::FitTo;
use egui_toast::Toasts;
use log::{debug, info};
use mc_bootstrap::ClientBootstrap;
use mc_downloader::prelude::{ClientDownloader, DownloadVersion};

use crate::{
    data::data_path,
    download_svc::{DownloadProgress, DownloadProgressMessage},
    resources::icon::Icon,
    settings::{LauncherInstance, LauncherSettings},
    widgets::{add_toast, GridWrapped, GridWrappedBuilder, IconButton, ProgressButton},
    MainState,
};

pub struct Instances {
    selected: RefCell<Option<LauncherInstance>>,
    frame_sizes: RefCell<Vec<Vec2>>,
    widget: GridWrapped<LauncherInstance>,
    download_button: ProgressButton,
    icon_close: Icon,
    progress: DownloadProgress,
    progress_rcv: Receiver<DownloadProgressMessage>,
}

impl Default for Instances {
    fn default() -> Self {
        let (progress, progress_rcv) = DownloadProgress::new();

        Self {
            progress,
            progress_rcv,
            selected: RefCell::new(None),
            frame_sizes: RefCell::new(Vec::new()),
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
            icon_close: Icon::from_svg("close.svg", FitTo::Size(24, 24)).unwrap(),
        }
    }
}

impl Instances {
    #[inline]
    pub fn show(
        &mut self,
        ui: &mut Ui,
        cfg: &mut LauncherSettings,
        widget: &mut CreateInstance,
        state: &mut MainState,
    ) {
        if !ui.is_rect_visible(ui.max_rect()) {
            return;
        }
        ui.add_space(20.);
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            let mut grid = self.widget.clone();
            let mut replaced = false;
            let mut reset = false;
            let selected = self.selected.clone().take();
            let mut grid_enabled = true;
            if selected.is_some() {
                ui.set_max_width(ui.available_width() - 300.);
                grid_enabled = !selected.unwrap().downloading;
            } else {
                ui.set_max_width(ui.available_width());
            }
            grid.set_enabled(grid_enabled)
                .set_items(cfg.instances.clone())
                .show(
                    ui,
                    Some(|| {
                        widget.reset();
                        reset = true;
                        state.create_instance = true;
                    }),
                    Some(|_: usize, item: &LauncherInstance, search: &str| {
                        item.name.to_lowercase().contains(&search.to_lowercase())
                    }),
                    |ui, i, item| {
                        let mut frame_sizes = self.frame_sizes.borrow_mut();
                        if frame_sizes.len() != cfg.instances.len() {
                            frame_sizes.resize(cfg.instances.len(), Vec2::default());
                        }
                        let i = i.clone();
                        ui.horizontal_centered(|ui| {
                            ui.add_space((ui.available_width() - frame_sizes[i].x) / 2.0);
                            let top_space = (ui.available_height() - frame_sizes[i].y) / 2.0;
                            let frame_response = ui.vertical_centered(|ui| {
                                ui.add_space(top_space);
                                let mut icon_path = item.path.clone();
                                icon_path.push_str("/icon.png");
                                ui.image(
                                    Icon::image_from_path(
                                        item.name.as_str(),
                                        icon_path.as_str(),
                                        egui_extras::image::FitTo::Original,
                                    )
                                    .unwrap()
                                    .id(ui.ctx()),
                                    (50., 50.),
                                );
                                ui.add_space(20.);
                                ui.label(RichText::new(item.name.clone()).size(20.));
                            });
                            frame_sizes[i] = frame_response.response.rect.size();
                        });
                    },
                    |s: usize| {
                        replaced = true;
                        self.selected.replace(Some(cfg.instances[s].clone()));
                    },
                );
            self.info_section(ui, cfg, &mut state.toasts, &mut grid);
            let launch_btn = if reset || replaced {
                self.download_button.set_progress(0.).clone()
            } else {
                self.download_button.clone()
            };
            self.download_button = launch_btn;
            if reset {
                grid.reset();
            }
            self.widget = grid.clone();
        });
    }

    #[inline]
    fn info_section(
        &mut self,
        ui: &mut Ui,
        cfg: &mut LauncherSettings,
        toasts: &mut Toasts,
        grid: &mut GridWrapped<LauncherInstance>,
    ) {
        let binding = self.selected.clone();
        let mut r_instance = binding.borrow_mut();
        SidePanel::right(format!("Instance Details - {:?}", r_instance))
            .exact_width(300.)
            .show_animated_inside(ui, r_instance.is_some(), |ui| {
                ui.vertical_centered_justified(|ui| {
                    if let Some(ref mut instance) = *r_instance {
                        if !instance.downloaded {
                            self.download_button.set_text("Start Download");
                        }
                        let mut icon_path = instance.path.clone();
                        icon_path.push_str("/icon.png");
                        // Close Button
                        ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                            ui.add_space(10.);
                            let btn_close =
                                ui.add_sized((20., 20.), IconButton::new(&self.icon_close));
                            if btn_close.clicked() && !instance.downloading {
                                info!("Close SidePanel clicked!!");
                                self.selected.replace(None);
                                grid.reset();
                            }
                        });
                        // instance image
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
                        // Name Instance
                        ui.label(RichText::new(instance.name.clone()).size(25.).strong());
                        if let Some(version) = instance.version.clone() {
                            ui.add_space(10.);
                            ui.label(version.get_version_id());
                        }
                        ui.add_space(20.);
                        // Buttons
                        self.action_buttons(ui, cfg, toasts);
                        ui.add_space(20.);
                    }
                });
            });
    }

    #[inline]
    fn action_buttons(&mut self, ui: &mut Ui, cfg: &mut LauncherSettings, toasts: &mut Toasts) {
        let mut binding = self.selected.borrow_mut();
        if binding.is_none() {
            return;
        }
        let mut_instance = binding.as_mut().unwrap();
        // Buttons
        ui.horizontal(|ui| {
            // progress for downloading
            if mut_instance.downloading {
                while let Ok(msg) = self.progress_rcv.try_recv() {
                    match msg {
                        DownloadProgressMessage::Setup(_) => add_toast(
                            toasts,
                            "Instance",
                            &format!(
                                "Downloading resources for instance: {}",
                                mut_instance.name.clone()
                            ),
                            crate::widgets::OpenMCToastKind::Info,
                        ),
                        DownloadProgressMessage::Update(curr, max) => {
                            ui.ctx().request_repaint();
                            self.download_button
                                .set_progress(curr as f32 / max as f32)
                                .set_text("Downloading...")
                                .build();
                        }
                        DownloadProgressMessage::End => {
                            debug!("Message Downloaded!!");
                            mut_instance.downloading = false;
                            mut_instance.downloaded = true;
                            self.download_button
                                .set_progress(0.)
                                .set_text("Launch")
                                .build();
                            add_toast(
                                toasts,
                                "Instance",
                                &format!(
                                    "Resources for instance are Downloaded: {}",
                                    mut_instance.name.clone()
                                ),
                                crate::widgets::OpenMCToastKind::Info,
                            );
                        }
                    }
                }
            }
            if mut_instance.downloaded {
                self.download_button.set_progress(0.).set_text("Launch");
            }
            // Launch
            let width = ui.available_width() - 10.;
            ui.add_enabled_ui(!mut_instance.downloading, |ui| {
                let btn = ui.add_sized(Vec2::new(width, 50.), self.download_button.clone());
                if btn.clicked() {
                    if !mut_instance.downloaded && !mut_instance.downloading {
                        let progress = self.progress.clone();
                        let v = mut_instance.version.clone().unwrap();
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
                        mut_instance.downloading = true;
                        self.download_button.set_text("Downloading...");
                    }
                    if mut_instance.downloaded {
                        let v = mut_instance.version.clone().unwrap();
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
        // Edit, Open and Delete Buttons
        ui.horizontal(|ui| {
            let width = ui.available_width() / 3. - 10.;

            ui.add_enabled_ui(false, |ui| {
                ui.add_sized(Vec2::new(width, 30.), Button::new("Edit").wrap(true));
            });
            let open_btn = ui.add_sized(Vec2::new(width, 30.), Button::new("Open").wrap(true));
            if open_btn.clicked() {
                open::that(mut_instance.path.clone()).unwrap();
            }
            ui.add_enabled_ui(!mut_instance.downloading, |ui| {
                let delete_btn = ui.add_sized(
                    Vec2::new(width, 30.),
                    Button::new("Delete").wrap(true).fill(Color32::LIGHT_RED),
                );
                if delete_btn.clicked() {
                    self.selected.replace(None);
                    cfg.remove_instance(mut_instance.name.clone());
                }
            });
        });
        cfg.instances.iter_mut().for_each(|i| {
            if i.name == mut_instance.name {
                *i = mut_instance.clone();
            }
        });
    }
}
