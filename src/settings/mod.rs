use crate::{
    args::{OpenMCArgs, OpenMCommands},
    data::{config_path, theme::ThemeType},
};
use clap::Parser;
use log::{debug, trace};
use mc_downloader::launcher_manifest::LauncherManifestVersion;
use serde::{Deserialize, Serialize};

mod load;
mod save;

pub use load::load_settings;
pub use save::save_settings;

// Data to save and load into preferences
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct LauncherSettings {
    pub theme: ThemeType,
    pub last_launched: Option<LauncherInstance>,
    pub instances: Vec<LauncherInstance>,
    #[serde(skip)]
    pub exists_icons: bool,
}

// Data of instance
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LauncherInstance {
    pub name: String,
    pub path: String,
    pub icon_path: String,
    pub version: Option<MinecraftVersion>,
}

// Specific version type and literal version
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

    pub fn add_instance(&mut self, instance: LauncherInstance) {
        if self.instances.is_empty() {
            self.last_launched = Some(instance.clone());
        }
        self.instances.push(instance);
        self.save();
    }

    pub fn remove_instance(&mut self, name: String) {
        if let Some(pos) = self.instances.iter().position(|i| i.name == name) {
            self.instances.remove(pos);
            if self.last_launched.is_some() {
                self.last_launched = None;
            }
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
