#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use data::APP_NAME;
use download_svc::create_icons_svc;
use egui_stylist::StylistState;
use openmc_lite::{
    data, download_svc, resources,
    screens::{self, AccountType, Instances},
    settings, widgets, MainState,
};
use resources::ResourceLoader;
use screens::{tab_buttons, CreateInstance, ViewType};
use settings::LauncherSettings;
use widgets::{open_file_dialog, TitleBar};

#[cfg(feature = "inspect")]
use egui_inspect::EguiInspect;

use mc_downloader::prelude::{ClientDownloader, DownloaderService};

fn main() -> Result<(), eframe::Error> {
    env_logger::Builder::from_env(env_logger::Env::new().filter_or("OPENMC_LOG", "warn"))
        .format_timestamp(None)
        .init();
    let icon_data = {
        let image = image::load_from_memory(include_bytes!("../assets/app.png"))
            .expect("failed to load icon");
        let image_buffer = image.to_rgba8();
        eframe::IconData {
            width: image.width(),
            height: image.height(),
            rgba: image_buffer.into_vec(),
        }
    };

    let options = eframe::NativeOptions {
        decorated: false,
        centered: true,
        // transparent: true,
        initial_window_size: Some(eframe::egui::vec2(1080., 720.)),
        min_window_size: Some(eframe::egui::vec2(1080., 720.)),
        icon_data: Some(icon_data),
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
    titlebar: TitleBar,
    curr_view: ViewType,
    instances_widget: Instances,
    create_widget: CreateInstance,
    downloader: Option<DownloaderService>,
    state: MainState,
}

impl MainApplication {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let launcher_config = LauncherSettings::new();
        let mut theme = launcher_config.theme.apply(&cc.egui_ctx);
        theme.set_file_dialog_function(Box::new(open_file_dialog));
        log::debug!("Theme Loaded {:?}", launcher_config.theme);

        let mc = ClientDownloader::new().unwrap();

        Self {
            launcher_config: launcher_config.clone(),
            theme,
            state: MainState::default(),
            resources: ResourceLoader::default(),
            instances_widget: Instances::default(),
            create_widget: CreateInstance::new(&mc),
            titlebar: TitleBar::default(),
            downloader: if !launcher_config.exists_icons {
                Some(create_icons_svc())
            } else {
                None
            },
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
        widgets::CentralPanel::default().show(ctx, |ui| {
            #[cfg(feature = "inspect")]
            egui::Window::new("(Debug) Stats")
                .title_bar(true)
                .movable(true)
                .resizable(true)
                .collapsible(true)
                .default_open(false)
                .default_pos(ui.max_rect().center())
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.collapsing("State", |ui| {
                            self.state.inspect_mut("App State", ui);
                        });

                        ui.collapsing("Launcher Settings", |ui| {
                            self.launcher_config.inspect_mut("Settings", ui);
                        });
                    });
                });

            ui.vertical(|ui| {
                self.titlebar.draw_title_bar_ui(
                    ui,
                    frame,
                    ui.max_rect(),
                    &mut self.state,
                    &mut self.launcher_config,
                    &mut self.downloader,
                );
                ui.add_space(10.);
                if !self.state.create_instance {
                    tab_buttons(ui, &mut self.curr_view);
                    ui.add_space(10.);
                    match self.curr_view {
                        ViewType::Home => screens::home(ui, &self.launcher_config, &self.resources),
                        ViewType::Instances => self.instances_widget.show(
                            ui,
                            &mut self.launcher_config,
                            &mut self.create_widget,
                            &mut self.state,
                        ),
                        ViewType::Preferences => {
                            screens::preferences(ui, &mut self.theme, &mut self.launcher_config)
                        }
                    }
                } else {
                    self.state.sub_title = "Create Instance".to_string();
                    self.create_widget.show(
                        ui,
                        &mut self.theme,
                        &mut self.launcher_config,
                        &mut self.state,
                    );
                }
            });
            let modal = self.state.modal.clone();
            modal.show(ui, |ui| {
                let account_type = if self.launcher_config.session.is_logged() {
                    Some(AccountType::from(
                        self.launcher_config.session.account_origin(),
                    ))
                } else {
                    None
                };
                screens::Account::new(account_type).show(
                    ui,
                    &self.resources,
                    &mut self.state,
                    &mut self.launcher_config,
                );
            });
            // Toasts/Notification Area
            self.state.toasts.show(ctx);
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.launcher_config.save();
    }
}
