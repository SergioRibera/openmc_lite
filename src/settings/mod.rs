use std::fmt::Debug;

use crate::{
    args::{OpenMCArgs, OpenMCommands},
    data::{config_path, data_path, theme::ThemeType},
};
use clap::Parser;
use log::{debug, info, trace};
use mc_downloader::launcher_manifest::LauncherManifestVersion;
use serde::{Deserialize, Serialize};

#[cfg(feature = "inspect")]
use egui::Color32;
#[cfg(feature = "inspect")]
use egui_inspect::EguiInspect;

mod load;
mod save;

pub use load::load_settings;
pub use save::save_settings;

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "inspect", derive(EguiInspect))]
pub struct UserSession {
    pub name: String,
    pub uuid: String,
    pub access_token: String,
    #[cfg_attr(feature = "inspect", inspect(hide))]
    pub origin: String,
}

impl Default for UserSession {
    fn default() -> Self {
        let d = "null";
        let name = {
            let n = names::Generator::default().next().unwrap();
            let sp = n.split('-').collect::<Vec<&str>>();
            sp.iter()
                .map(|s| {
                    let mut chars = s.chars();
                    format!(
                        "{}{}",
                        chars.nth(0).unwrap().to_uppercase(),
                        chars
                            .map(|c| c.to_string())
                            .collect::<Vec<String>>()
                            .join("")
                    )
                })
                .collect::<Vec<String>>()
                .join(" ")
        };
        Self {
            name,
            origin: String::new(),
            uuid: d.to_string(),
            access_token: d.to_string(),
        }
    }
}

impl UserSession {
    pub fn is_logged(&self) -> bool {
        let d = "null";
        !self.name.is_empty() && self.uuid != d.to_string() && self.access_token != d.to_string()
    }

    pub fn account_origin(&self) -> String {
        if self.origin.is_empty() || !self.is_logged() {
            return String::from("LOCAL");
        }
        self.origin.clone()
    }
}

// Data to save and load into preferences
#[derive(Default, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "inspect", derive(EguiInspect))]
pub struct LauncherSettings {
    pub theme: ThemeType,
    #[serde(default)]
    pub session: UserSession,
    #[cfg_attr(feature = "inspect", inspect(hide, custom_func_mut = "custom_instance_inspect"))]
    pub last_launched: Option<LauncherInstance>,
    pub instances: Vec<LauncherInstance>,
    #[serde(skip)]
    pub exists_icons: bool,
}

// Data of instance
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "inspect", derive(EguiInspect))]
pub struct LauncherInstance {
    pub name: String,
    pub path: String,
    #[cfg_attr(
        feature = "inspect",
        inspect(hide, custom_func_mut = "custom_mc_version_inspect")
    )]
    pub version: Option<MinecraftVersion>,
    pub downloaded: bool,
    #[serde(skip)]
    pub downloading: bool,
}

// Specific version type and literal version
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum MinecraftVersion {
    Release(String),
    Snapshot(String),
    OldBeta(String),
    OldAlpha(String),
}

impl LauncherSettings {
    pub fn new() -> Self {
        let opts = OpenMCArgs::parse();
        let mut cfg = load_settings::<Self>("launcher.conf");

        if let Some(cmd) = opts.command {
            match cmd {
                OpenMCommands::Launch { instance } => {
                    // @TODO: launch instance
                    if let Some(instance) = cfg.instances.iter().find(|i| i.name == instance) {
                        println!("Launching Instance '{}'", instance.name);
                        std::process::exit(0);
                    } else {
                        println!("Instance '{}' not exists", instance);
                        std::process::exit(1);
                    }
                }
            }
        }

        let p = config_path("icons");
        cfg.exists_icons = p.is_dir() && p.read_dir().unwrap().count() > 1;

        if let Some(t) = opts.theme {
            cfg.theme = t;
        }

        cfg
    }

    pub fn add_instance(
        &mut self,
        instance: LauncherInstance,
        icon_path: String,
    ) -> std::path::PathBuf {
        let name = instance.name.clone();
        let folder_name = format!("instances/{}", name);
        let path = data_path(folder_name.as_str());
        data_path(format!("{folder_name}/mods").as_str());
        data_path(format!("{folder_name}/resourcepacks").as_str());
        data_path(format!("{folder_name}/saves").as_str());
        data_path(format!("{folder_name}/shaderpacks").as_str());
        debug!(
            "The new path of instance: {path:?} - Exists: {}",
            path.exists()
        );
        let new_icon_path = {
            let mut path = path.clone();
            path.push("icon.png");
            path
        };
        std::fs::copy(icon_path, new_icon_path.clone()).unwrap();
        info!("Icon copied Succesfull: {new_icon_path:?}");
        let instance = LauncherInstance {
            path: path.to_str().unwrap().to_string(),
            ..instance
        };
        debug!("New LauncherInstance Information: {instance:?}");
        if self.instances.is_empty() || self.last_launched.is_none() {
            self.last_launched = Some(instance.clone());
        }
        self.instances.push(instance);
        info!("Instance pushed to instances list");
        self.save();
        path
    }

    pub fn remove_instance(&mut self, name: String) {
        if let Some(pos) = self.instances.iter().position(|i| i.name == name) {
            self.instances.remove(pos);
            if self.last_launched.is_some() {
                self.last_launched = None;
            }
            remove_instance_folder(name.as_str());
            self.save();
        }
    }

    pub fn save(&self) -> bool {
        trace!("Save Setting");
        let b = save_settings::<Self>(self.clone(), "launcher.conf");
        debug!("Settings Saved: {b}");
        b
    }
}

impl ToString for LauncherInstance {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Debug for MinecraftVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl MinecraftVersion {
    pub fn get_version_id(&self) -> String {
        match self {
            MinecraftVersion::Release(v) => v.clone(),
            MinecraftVersion::Snapshot(v) => v.clone(),
            MinecraftVersion::OldBeta(v) => v.clone(),
            MinecraftVersion::OldAlpha(v) => v.clone(),
        }
    }

    pub fn get_version_type(&self) -> String {
        match self {
            MinecraftVersion::Release(_) => "release".to_string(),
            MinecraftVersion::Snapshot(_) => "snapshot".to_string(),
            MinecraftVersion::OldBeta(_) => "old_beta".to_string(),
            MinecraftVersion::OldAlpha(_) => "old_alpha".to_string(),
        }
    }
}

impl ToString for MinecraftVersion {
    fn to_string(&self) -> String {
        match self {
            MinecraftVersion::Release(v) => format!("{v} (Release)"),
            MinecraftVersion::Snapshot(v) => format!("{v} (Snapshot)"),
            MinecraftVersion::OldBeta(v) => format!("{v} (Old Beta)"),
            MinecraftVersion::OldAlpha(v) => format!("{v} (Old Alpha)"),
        }
    }
}

impl From<&LauncherManifestVersion> for MinecraftVersion {
    fn from(v: &LauncherManifestVersion) -> Self {
        match v.version_type.as_str() {
            "release" => Self::Release(v.id.clone()),
            "snapshot" => Self::Snapshot(v.id.clone()),
            "old_beta" => Self::OldBeta(v.id.clone()),
            _ => Self::OldAlpha(v.id.clone()),
        }
    }
}

fn remove_instance_folder(name: &str) {
    let mut dir = data_path("instances");
    dir.push(name);

    if dir.exists() && dir.is_dir() {
        std::fs::remove_dir_all(dir).unwrap();
    }
}

#[cfg(feature = "inspect")]
impl EguiInspect for MinecraftVersion {
    fn inspect(&self, label: &'static str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.label(self.to_string());
        });
    }
    fn inspect_mut(&mut self, label: &'static str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.colored_label(Color32::from_rgb(255, 0, 0), self.to_string())
                .on_hover_text("inspect_mut is not implemented for MinecraftVersion");
        });
    }
}

#[cfg(feature = "inspect")]
fn custom_instance_inspect(
    value: &mut Option<LauncherInstance>,
    label: &'static str,
    ui: &mut egui::Ui,
) {
    if let Some(v) = value {
        v.inspect_mut(label, ui);
    } else {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.label("None");
        });
    }
}

#[cfg(feature = "inspect")]
fn custom_mc_version_inspect(
    value: &mut Option<MinecraftVersion>,
    label: &'static str,
    ui: &mut egui::Ui,
) {
    if let Some(v) = value {
        v.inspect_mut(label, ui);
    } else {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.label("None");
        });
    }
}
