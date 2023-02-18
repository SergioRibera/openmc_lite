use crate::{
    args::{OpenMCArgs, OpenMCommands},
    data::theme::ThemeType,
};
use clap::Parser;
use log::{debug, trace};
use serde::{Deserialize, Serialize};

mod load;
mod save;

pub use load::load_settings;
pub use save::save_settings;

// Data to save and load into preferences
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct LauncherSettings {
    pub(crate) theme: ThemeType,
    pub(crate) last_launched: Option<LauncherInstance>,
    pub(crate) instances: Vec<LauncherInstance>,
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

        if let Some(t) = opts.theme {
            cfg.theme = t;
        }

        cfg
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
