mod theme_type;

use marcel::{serial::Theme as SerializeTheme, Theme};
use once_cell::sync::Lazy;
pub use theme_type::*;

pub static LIGHT: Lazy<Theme> = Lazy::new(|| {
    let theme_str = include_str!("../../../assets/themes/light.theme.toml");
    let ser_theme = toml::from_str::<SerializeTheme>(theme_str).unwrap();
    Theme::parse(&ser_theme).unwrap()
});

pub static DARK: Lazy<Theme> = Lazy::new(|| {
    let theme_str = include_str!("../../../assets/themes/dark.theme.toml");
    let ser_theme = toml::from_str::<SerializeTheme>(theme_str).unwrap();
    Theme::parse(&ser_theme).unwrap()
});
