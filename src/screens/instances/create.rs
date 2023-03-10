use std::{cell::RefCell, path::PathBuf};

use egui::{Color32, Layout, RichText};
use egui_extras::Size;
use egui_stylist::StylistState;
use log::info;
use mc_downloader::prelude::ClientDownloader;

use crate::{
    data::config_path,
    resources::icon::Icon,
    settings::{LauncherSettings, MinecraftVersion},
    widgets::{add_toast, GridWrapped, GridWrappedBuilder, Steps, Tabs},
    MainState,
};

use super::utils::select_icon;

type StepCallback = fn(&mut CreateInstance, &mut StylistState, &mut egui::Ui);
type StepValidationCallback = fn(&mut CreateInstance, &mut LauncherSettings) -> Result<(), String>;

static STEPS: &[(&str, StepCallback, StepValidationCallback)] = &[
    ("Name", set_name, validate_name),
    ("Icon", set_icon, validate_icon),
    ("Version", set_version, validate_version),
];

pub struct CreateInstance {
    curr_step: u8,
    max_step: u8,
    icons: Vec<(String, Icon)>,
    grid: GridWrapped<u8>,
    versions: GridWrapped<String>,
    steps: Steps,
    tabs_versions: Tabs<(u8, Vec<String>)>,
    name: String,
    icon_selected: String,
    version_selected: Option<MinecraftVersion>,
}

impl CreateInstance {
    pub fn new(mc: &ClientDownloader) -> Self {
        let path_icons = config_path("icons");

        let icons = path_icons
            .read_dir()
            .unwrap()
            .flatten()
            .filter(|f| f.file_name().to_str().unwrap().ends_with(".png"))
            .flat_map(|f| {
                let path = f.path();
                let path = path.to_str().unwrap();
                let path = path.to_string();
                let icon = Icon::image_from_path(
                    f.file_name().to_str().unwrap(),
                    path.as_str(),
                    egui_extras::image::FitTo::Size(80, 80),
                );
                match icon {
                    Ok(icon) => Ok((path, icon)),
                    Err(e) => Err(e),
                }
            })
            .collect::<Vec<(String, Icon)>>();
        let icons_len = icons.len() as u8;

        let versions = mc.get_list_versions();

        let mc_releases = versions
            .iter()
            .filter(|v| v.version_type == "release")
            .map(|v| v.id.clone())
            .collect();
        let mc_snapshot = versions
            .iter()
            .clone()
            .filter(|v| &v.version_type == "snapshot")
            .map(|v| v.id.clone())
            .collect();
        let mc_oldbeta = versions
            .iter()
            .clone()
            .filter(|v| &v.version_type == "old_beta")
            .map(|v| v.id.clone())
            .collect();
        let mc_oldalpha = versions
            .iter()
            .filter(|v| &v.version_type == "old_alpha")
            .map(|v| v.id.clone())
            .collect();

        Self {
            icons,
            curr_step: 0,
            max_step: STEPS.len() as u8 - 1,
            steps: Steps::with_steps(STEPS.iter().map(|(n, _, _)| *n).collect()),
            grid: GridWrappedBuilder::default()
                .set_enabled(true)
                .show_search()
                .set_items((0u8..icons_len).collect::<Vec<u8>>())
                .set_cell_size((100., 100.))
                .set_button_text("Custom")
                .build(),
            versions: GridWrappedBuilder::default()
                .show_search()
                .set_enabled(true)
                .build(),
            tabs_versions: Tabs::new(
                &[
                    ("Release", (0u8, mc_releases)),
                    ("Snapshots", (1u8, mc_snapshot)),
                    ("Old Beta", (2u8, mc_oldbeta)),
                    ("Old Alpha", (3u8, mc_oldalpha)),
                ],
                0,
                20,
                Color32::WHITE,
            ),
            name: String::new(),
            icon_selected: String::new(),
            version_selected: None,
        }
    }

    pub fn reset(&mut self) {
        self.curr_step = 0;
        self.version_selected = None;
        self.icon_selected = String::new();
        self.name = String::new();
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        theme: &mut StylistState,
        cfg: &mut LauncherSettings,
        state: &mut MainState,
    ) {
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
                                    ui.add(self.steps.set_current(self.curr_step as usize).clone());
                                });
                            });
                            strip.empty();
                        });
                });
                strip.cell(|ui| {
                    let i = self.curr_step as usize;
                    STEPS[i].1(self, theme, ui);
                });
                strip.cell(|ui| {
                    ui.add_space(10.);
                    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                        next_prev_btn(ui, self, cfg, state);
                    });
                });
            });
    }
}

fn next_prev_btn(
    ui: &mut egui::Ui,
    ctx: &mut CreateInstance,
    cfg: &mut LauncherSettings,
    state: &mut MainState,
) {
    let size = egui::Vec2::new(70., 40.);
    if ctx.curr_step == ctx.max_step {
        let btn = ui.add(
            eframe::egui::Button::new(RichText::new("Finish").size(20.))
                .min_size(size)
                .wrap(true),
        );
        if btn.clicked() {
            state.sub_title = String::new();
            state.create_instance = false;
            cfg.add_instance(
                crate::settings::LauncherInstance {
                    name: ctx.name.clone(),
                    version: ctx.version_selected.clone(),
                    ..Default::default()
                },
                ctx.icon_selected.clone(),
            );
        }
    } else if ui
        .add(
            eframe::egui::Button::new(RichText::new("Next").size(20.))
                .min_size(size)
                .wrap(true),
        )
        .clicked()
    {
        match STEPS[ctx.curr_step as usize].2(ctx, cfg) {
            Ok(_) => ctx.curr_step += 1,
            Err(e) => add_toast(
                &mut state.toasts,
                "Invalid Input",
                e.as_str(),
                crate::widgets::OpenMCToastKind::Error,
            ),
        }
    }
    if ctx.curr_step > 0
        && ui
            .add(
                eframe::egui::Button::new(RichText::new("Prev").size(20.))
                    .min_size(size)
                    .wrap(true),
            )
            .clicked()
    {
        ctx.curr_step -= 1;
    }
}

fn create_label(ui: &mut egui::Ui, content: &str) {
    ui.label(RichText::new(content).fallback_text_style(egui::TextStyle::Body));
    ui.add_space(10.);
}

fn set_name(data: &mut CreateInstance, _theme: &mut StylistState, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(30.);
        create_label(ui, "What Name have for your instance?");
        ui.add(egui::TextEdit::singleline(&mut data.name).hint_text("My Best Minecraft Instance"));
    });
}

fn validate_name(data: &mut CreateInstance, cfg: &mut LauncherSettings) -> Result<(), String> {
    if data.name.is_empty() {
        return Err("The name cannot empty".to_string());
    }
    if data.name.len() < 3 {
        return Err("The name size need more than 5".to_string());
    }
    if cfg.instances.iter().any(|i| i.name == data.name) {
        return Err("An instance of this name already exists".to_string());
    }
    Ok(())
}

fn set_icon(data: &mut CreateInstance, theme: &mut StylistState, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        // Icon
        create_label(ui, "Choose an icon that characterizes your instance");
        ui.add_space(20.);
        let mut grid = data.grid.clone();
        let selected = RefCell::new(String::new());
        grid.show(
            ui,
            Some(|| {
                if let Some(icon) = select_icon(theme) {
                    selected.replace(icon.0);
                }
            }),
            Some(|i: usize, _: &u8, search: &str| {
                data.icons[i]
                    .0
                    .to_lowercase()
                    .contains(&search.to_lowercase())
            }),
            |ui, _, v| {
                ui.centered_and_justified(|ui| {
                    ui.image(data.icons[*v as usize].1.id(ui.ctx()), (50., 50.));
                });
            },
            |s: usize| {
                selected.replace(data.icons[s].0.clone());
            },
        );
        let selected = selected.borrow();
        if !selected.is_empty() {
            data.icon_selected = selected.clone();
            info!("Icon is clicked; Path: {}", data.icon_selected);
        }
        data.grid = grid;
    });
}
fn validate_icon(data: &mut CreateInstance, _cfg: &mut LauncherSettings) -> Result<(), String> {
    if data.icon_selected.is_empty() {
        return Err("Please select one icon".to_string());
    }
    let path = PathBuf::from(&data.icon_selected);
    if !path.is_file() && !path.exists() {
        return Err("The icon file not exists".to_string());
    }
    Ok(())
}

fn set_version(data: &mut CreateInstance, _theme: &mut StylistState, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        create_label(ui, "Choose an icon that characterizes your instance");
        ui.add_space(10.);
        let mut grid = data.versions.clone();
        let (n, tab_content) = data.tabs_versions.show(ui);
        let selected = RefCell::new(String::new());
        ui.add_space(20.);
        grid.set_cell_size((ui.available_width() - 20., 30.))
            .set_items(tab_content.clone())
            .show(
                ui,
                None::<fn()>,
                Some(|_: usize, item: &String, search: &str| {
                    item.to_string()
                        .to_lowercase()
                        .contains(&search.to_lowercase())
                }),
                |ui, _i, item| {
                    ui.horizontal(|ui| {
                        ui.label(item.to_string());
                    });
                },
                |s| {
                    selected.replace(tab_content[s].clone());
                },
            );
        let selected = selected.borrow();
        if !selected.is_empty() {
            data.version_selected = match n {
                0 => Some(MinecraftVersion::Release(selected.clone())),
                1 => Some(MinecraftVersion::Snapshot(selected.clone())),
                2 => Some(MinecraftVersion::OldBeta(selected.clone())),
                3 => Some(MinecraftVersion::OldAlpha(selected.clone())),
                _ => None,
            };
            info!("Version Selected: {:?}", data.version_selected);
        }
        data.versions = grid;
    });
}

fn validate_version(data: &mut CreateInstance, _cfg: &mut LauncherSettings) -> Result<(), String> {
    if data.version_selected.is_none() {
        return Err("Please select one version".to_string());
    }
    Ok(())
}
