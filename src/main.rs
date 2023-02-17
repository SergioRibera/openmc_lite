#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use data::APP_NAME;
use egui_stylist::StylistState;
use resources::ResourceLoader;
use screens::tab_buttons;
use screens::ViewType;
use settings::{LauncherInstance, LauncherSettings};
use widgets::{AppComponent, TitleBar};

mod args;
mod data;
mod resources;
mod screens;
mod settings;
mod widgets;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        decorated: false,
        initial_window_size: Some(eframe::egui::vec2(1080., 720.)),
        min_window_size: Some(eframe::egui::vec2(1080., 720.)),
        centered: true,
        default_theme: eframe::Theme::Light,
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
    // windows_state: WindowEvent,
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum WindowEvent {
//     Normal,
//     Minimize,
//     Maximize,
//     Close,
// }
//
// #[derive(Debug, Clone)]
// pub enum MainMessage {
//     None,
//     ThemeChanged(ThemeType),
//     ViewChanged(ViewType),
//     WindowEvent(WindowEvent),
//
//     InstanceView(InstanceEvent),
//
//     LauncherInstanceChanged(LauncherInstance),
// }

impl MainApplication {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let launcher_config = LauncherSettings::new();
        let theme = launcher_config.theme.apply();
        let (style, font_definitions) = theme.export_theme().extract();
        cc.egui_ctx.set_style(style);
        cc.egui_ctx.set_fonts(font_definitions);

        Self {
            launcher_config: launcher_config.clone(),
            curr_step: None,
            theme,
            resources: ResourceLoader::new(&cc.egui_ctx),
            curr_view: if launcher_config.instances.is_empty() {
                ViewType::Instances
            } else {
                ViewType::Home
            },
            // windows_state: WindowEvent::Normal,
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
                        ViewType::Instances => screens::instances(ui, &self.launcher_config),
                        ViewType::Preferences => screens::preferences(ui, &mut self.theme, &self.launcher_config),
                        _ => (),
                    }
                } else {
                }
            });
        });
    }
}
