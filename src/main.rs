#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use data::APP_NAME;
use egui_stylist::StylistState;
use egui_toast::Toasts;
use resources::ResourceLoader;
use screens::tab_buttons;
use screens::ViewType;
use settings::LauncherSettings;
use widgets::open_file_dialog;
use widgets::{AppComponent, TitleBar};

use crate::widgets::create_toast;

mod args;
mod data;
mod resources;
mod screens;
mod settings;
mod widgets;

fn main() -> Result<(), eframe::Error> {
    env_logger::Builder::from_env(env_logger::Env::new().filter_or("OPENMC_LOG", "warn"))
        .format_timestamp(None)
        .init();
    let options = eframe::NativeOptions {
        decorated: false,
        initial_window_size: Some(eframe::egui::vec2(1080., 720.)),
        min_window_size: Some(eframe::egui::vec2(1080., 720.)),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Box::new(MainApplication::new(cc))),
    )
}

pub struct MainApplication {
    launcher_config: LauncherSettings,
    resources: ResourceLoader,
    theme: StylistState,
    curr_view: ViewType,
    curr_step: Option<u8>,
    toasts: Toasts,
}

impl MainApplication {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let launcher_config = LauncherSettings::new();
        let mut theme = launcher_config.theme.apply(&cc.egui_ctx);
        theme.set_file_dialog_function(Box::new(open_file_dialog));
        log::debug!("Theme Loaded {:?}", launcher_config.theme);

        Self {
            launcher_config: launcher_config.clone(),
            curr_step: None,
            theme,
            resources: ResourceLoader::new(&cc.egui_ctx),
            toasts: create_toast(),
            curr_view: if launcher_config.instances.is_empty() {
                ViewType::Instances
            } else {
                ViewType::Home
            },
        }
    }
}

impl eframe::App for MainApplication {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                TitleBar::with_frame(self, ui, frame);
                ui.add_space(20.);
                if self.curr_step.is_none() {
                    tab_buttons(ui, &mut self.curr_view);
                    ui.add_space(10.);
                    match self.curr_view {
                        ViewType::Home => screens::home(ui, &self.launcher_config, &self.resources),
                        ViewType::Instances => {
                            screens::instances(ui, &self.launcher_config)
                        }
                        ViewType::Preferences => {
                            screens::preferences(ui, &mut self.theme, &mut self.launcher_config)
                        }
                        _ => (),
                    }
                } else {
                }
            });
            // Toasts/Notification Area
            self.toasts.show(ctx);
        });
    }
}
