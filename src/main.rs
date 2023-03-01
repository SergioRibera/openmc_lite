#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use data::APP_NAME;
use download_svc::create_icons_svc;
use egui_stylist::StylistState;
use openmc_lite::{
    data, download_svc, resources,
    screens::{self, Instances},
    settings, widgets, MainState,
};
use resources::ResourceLoader;
use screens::{tab_buttons, CreateInstance, ViewType};
use settings::LauncherSettings;
use widgets::{open_file_dialog, TitleBar};

use mc_downloader::prelude::{ClientDownloader, DownloaderService};

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
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                self.titlebar.draw_title_bar_ui(
                    ui,
                    frame,
                    ui.max_rect(),
                    &mut self.state,
                    &mut self.downloader,
                );
                ui.add_space(20.);
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
                        _ => (),
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
            // Toasts/Notification Area
            self.state.toasts.show(ctx);
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.launcher_config.save();
    }
}
