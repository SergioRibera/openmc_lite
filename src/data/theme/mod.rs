mod theme_type;

use std::path::PathBuf;

use eframe::egui::Context;
use egui_stylist::StylistState;
use egui_theme::EguiTheme;
use once_cell::sync::Lazy;
pub use theme_type::*;

pub static LIGHT: Lazy<StylistState> = Lazy::new(|| {
    let theme_str = include_str!("../../../assets/themes/light.theme.toml");
    toml::from_str::<StylistState>(theme_str).unwrap()
});

pub static DARK: Lazy<StylistState> = Lazy::new(|| {
    let theme_str = include_str!("../../../assets/themes/dark.theme.toml");
    toml::from_str::<StylistState>(theme_str).unwrap()
});
