#![allow(dead_code)]

use litcrypt::lc;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub mod theme;

pub static APP_NAME: &str = "OpenMC Lite";
pub static APP_FOLDER: &str = "openmc_lite";
pub static APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: APP_FOLDER,
    author: "SergioRibera",
};

pub static OPENMC_SECURE_KEY: Lazy<String> = Lazy::new(|| lc!(env!("OPENMC_ENCRYPT_KEY")));

// on linux: ~/.config/{app_info.name}/{path}
/// This function only works with folders
pub fn config_path(path: &str) -> PathBuf {
    let res = app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, path).unwrap();
    if !res.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
    res
}

// on linux: ~/.local/share/{app_info.name}/{path}
/// This function only works with folders
pub fn data_path(path: &str) -> PathBuf {
    let res = app_dirs::app_dir(app_dirs::AppDataType::UserData, &APP_INFO, path).unwrap();
    if !res.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
    res
}
