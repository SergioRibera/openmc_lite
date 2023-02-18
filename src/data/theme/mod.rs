mod theme_type;

use egui::Context;
use egui_stylist::{StylistFileDialog, StylistState};
use log::{trace, debug};
use once_cell::sync::Lazy;
pub use theme_type::*;

use crate::settings::LauncherSettings;

pub static LIGHT: Lazy<StylistState> = Lazy::new(|| {
    let theme_str = include_str!("../../../assets/themes/light.theme.toml");
    trace!("Loading Light Theme");
    toml::from_str::<StylistState>(theme_str).unwrap()
});

pub static DARK: Lazy<StylistState> = Lazy::new(|| {
    let theme_str = include_str!("../../../assets/themes/dark.theme.toml");
    trace!("Loading Dark Theme");
    toml::from_str::<StylistState>(theme_str).unwrap()
});

pub fn save_theme(state: &mut StylistState, cfg: &mut LauncherSettings, ctx: &Context) {
    if let Some(path) =
        state.file_dialog(StylistFileDialog::Save, Some(("theme", &["toml", "theme"])))
    {
        debug!("Selected Path where Save: {path:?}");
        {
            let path = path.to_str().unwrap();
            cfg.theme = ThemeType::Custom((path.to_string(), state.clone()));
            cfg.theme.apply(ctx);
            trace!("Theme applied");
            cfg.save();
            trace!("Launcher configs saved");
        }

        let theme_str = toml::ser::to_string_pretty::<StylistState>(state).unwrap();
        std::fs::write(path, theme_str).unwrap();
        trace!("Theme file saved");
    }
}

pub fn load_theme(state: &mut StylistState, cfg: &mut LauncherSettings, ctx: &Context) {
    if let Some(path) =
        state.file_dialog(StylistFileDialog::Open, Some(("theme", &["toml", "theme"])))
    {
        debug!("Selected Path from load: {path:?}");
        {
            let path = path.to_str().unwrap();
            cfg.theme = ThemeType::Custom((path.to_string(), state.clone()));
            cfg.theme.apply(ctx);
            trace!("Theme applied");
            cfg.save();
            trace!("Launcher configs saved");
        }
        let theme_str = std::fs::read_to_string(&path).unwrap();
        let callback = state.file_dialog_function.clone();
        *state = toml::from_str::<StylistState>(theme_str.as_str()).unwrap();
        trace!("Replace StylistState from loaded file");
        if callback.is_some() {
            trace!("Set file dialog callback");
            state.file_dialog_function = callback;
        }
    }
}
