use std::sync::{mpsc::Receiver, Arc, Mutex};

use eframe::egui;
use egui::{Align, Align2, Color32, FontId, Id, Layout, Sense, Stroke};
use log::{debug, info};
use mc_downloader::prelude::DownloaderService;

use crate::{
    data::{theme::ThemeType, APP_NAME},
    download_svc::{DownloadProgress, DownloadProgressMessage},
    resources::Icons,
    settings::LauncherSettings,
    widgets::add_toast,
    MainState,
};

use super::IconButton;

pub struct TitleBar {
    resources: Icons,
    start_download: bool,
    curr_progress: f32,
    progress: DownloadProgress,
    progress_rcv: Receiver<DownloadProgressMessage>,
}

impl Default for TitleBar {
    fn default() -> Self {
        let (progress, progress_rcv) = DownloadProgress::new();

        Self {
            progress,
            progress_rcv,
            curr_progress: 0.,
            start_download: false,
            resources: Icons::preload().unwrap(),
        }
    }
}

impl TitleBar {
    pub fn draw_title_bar_ui(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        app_rect: egui::Rect,
        state: &mut MainState,
        cfg: &mut LauncherSettings,
        downloader: &mut Option<DownloaderService>,
    ) {
        let title_bar_height = 32.0;
        let pb_height = 3.;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };

        let mut pb_rect = title_bar_rect;
        pb_rect.min.y = pb_rect.max.y + 5.;
        pb_rect.max.y += pb_height;
        let title = if !state.sub_title.is_empty() {
            format!("{APP_NAME} - {}", state.sub_title)
        } else {
            APP_NAME.to_string()
        };

        let painter = ui.painter();

        let title_bar_response = ui.interact(
            title_bar_rect,
            Id::new("title_bar"),
            Sense::click_and_drag(),
        );

        // Paint the title:
        painter.text(
            title_bar_rect.center(),
            Align2::CENTER_CENTER,
            title,
            FontId::proportional(20.0),
            ui.style().visuals.text_color(),
        );

        // Interact with the title bar (drag to move window):
        if title_bar_response.double_clicked() {
            frame.set_maximized(!frame.info().window_info.maximized);
        } else if title_bar_response.is_pointer_button_down_on() {
            frame.drag_window();
        }

        // User Profile
        self.show_profile(title_bar_rect, ui, cfg, state);

        // Windows Controlls
        ui.allocate_ui_at_rect(title_bar_rect, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.visuals_mut().button_frame = false;
                ui.add_space(8.0);

                let close_btn = ui
                    .add(IconButton::new(&self.resources.close))
                    .on_hover_text("Close Window");
                if close_btn.clicked() {
                    cfg.save();
                    frame.close();
                };
                if !frame.is_web() {
                    if frame.info().window_info.maximized {
                        let maximized_response = ui
                            .add(IconButton::new(&self.resources.restore))
                            .on_hover_text("Restore window");
                        if maximized_response.clicked() {
                            frame.set_maximized(false);
                        }
                    } else {
                        let maximized_response = ui
                            .add(IconButton::new(&self.resources.maximize))
                            .on_hover_text("Maximize window");
                        if maximized_response.clicked() {
                            frame.set_maximized(true);
                        }
                    }

                    let minimized_response = ui
                        .add(IconButton::new(&self.resources.minimize))
                        .on_hover_text("Minimize the window");
                    if minimized_response.clicked() {
                        frame.set_minimized(true);
                    }
                }
                self.toggle_themes(ui, cfg);
            });
        });

        // bottom progressbar on titlebar
        if let Some(d) = downloader.clone() {
            if !self.start_download {
                info!("Start Download");
                self.start_download = true;
                let progress = self.progress.clone();
                std::thread::spawn(move || {
                    d.run(Some(Arc::new(Mutex::new(progress)))).unwrap();
                });
            }
        }
        if self.start_download {
            while let Ok(msg) = self.progress_rcv.try_recv() {
                match msg {
                    DownloadProgressMessage::Setup(_) => add_toast(
                        &mut state.toasts,
                        "Extra Assets",
                        "Downloading extra assets for launcher, ex: icons, sounds, etc",
                        crate::widgets::OpenMCToastKind::Info,
                    ),
                    DownloadProgressMessage::Update(curr, _max) => self.curr_progress = curr as f32,
                    DownloadProgressMessage::End => {
                        *downloader = None;
                        self.start_download = false;
                    }
                }
                ui.allocate_ui_at_rect(pb_rect, |ui| {
                    debug!("Painting Download Progress {}", self.curr_progress);
                    let painter = ui.painter();
                    let rect = ui.max_rect();
                    let pos = rect.left_center();
                    let max_width = rect.right_center().x;
                    let mut to = pos;
                    to.x += (self.curr_progress / 1394096.) * max_width;
                    painter.line_segment([pos, to], Stroke::new(1.5, Color32::LIGHT_BLUE));
                });
            }
        }
    }

    pub fn toggle_themes(&self, ui: &mut egui::Ui, cfg: &mut LauncherSettings) {
        let (icon, new_theme) = match cfg.theme {
            ThemeType::Light => (&self.resources.night_mode, ThemeType::Dark),
            ThemeType::Dark => (&self.resources.light_mode, ThemeType::Light),
            ThemeType::Custom(_) => (&self.resources.light_mode, ThemeType::Light),
        };

        let toggle_btn = ui
            .add_sized((24., 24.), IconButton::new(icon))
            .on_hover_text("Toggle Theme");
        if toggle_btn.clicked() {
            cfg.theme = new_theme;
            cfg.save();
        }
    }

    pub fn show_profile(
        &self,
        rect: egui::Rect,
        ui: &mut egui::Ui,
        cfg: &mut LauncherSettings,
        state: &mut MainState,
    ) {
        ui.allocate_ui_at_rect(rect, |ui| {
            let resp = ui
                .with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.add_space(10.);
                    ui.image(self.resources.app.id(ui.ctx()), (32., 32.));
                    ui.vertical(|ui| {
                        ui.label(cfg.session.name.clone());
                        ui.label(cfg.session.account_origin());
                    });
                    ui.image(self.resources.expand_arrow.id(ui.ctx()), (10., 10.));
                })
                .response;
            let resp = ui
                .interact(resp.rect, Id::new("__openmc__tabtitle"), Sense::click())
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text_at_pointer("Profile Settings");

            if resp.clicked() {
                info!("Profile clicked!");
                state.modal.open_modal(ui);
            }
        });
    }
}
