use std::{fmt::Debug, fs};

use clap::ValueEnum;
use egui::Context;
use egui_stylist::StylistState;
use log::debug;
use serde::{de::Visitor, Deserialize, Serialize};

#[cfg(feature = "inspect")]
use egui::Color32;
#[cfg(feature = "inspect")]
use egui_inspect::EguiInspect;

use crate::data::APP_INFO;

#[derive(Default, Clone, PartialEq)]
pub enum ThemeType {
    #[default]
    Light,
    Dark,
    Custom((String, StylistState)),
}

impl From<&str> for ThemeType {
    fn from(v: &str) -> Self {
        match v {
            "light" => ThemeType::Light,
            "dark" => ThemeType::Dark,
            path_str => {
                debug!("Reading theme from: {path_str}");
                let content = fs::read_to_string(path_str).unwrap();
                let theme = toml::from_str::<StylistState>(&content).unwrap();
                ThemeType::Custom((path_str.to_string(), theme))
            }
        }
    }
}

impl ThemeType {
    pub fn apply(&self, ctx: &Context) -> StylistState {
        let t = match self {
            ThemeType::Light => super::LIGHT.to_owned(),
            ThemeType::Dark => super::DARK.to_owned(),
            ThemeType::Custom((_, theme)) => theme.clone(),
        };
        let (style, font_definitions) = t.export_theme().extract();
        ctx.set_style(style);
        ctx.set_fonts(font_definitions);
        t
    }
}

impl Debug for ThemeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "Light"),
            Self::Dark => write!(f, "Dark"),
            Self::Custom((name, _)) => f.debug_tuple("Custom").field(name).finish(),
        }
    }
}

#[cfg(feature = "inspect")]
impl EguiInspect for ThemeType {
    fn inspect(&self, label: &'static str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.label(format!("{:?}", self));
        });
    }
    fn inspect_mut(&mut self, label: &'static str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(label.to_owned() + ":");
            ui.colored_label(Color32::from_rgb(255, 0, 0), format!("{:?}", self))
                .on_hover_text("inspect_mut is not implemented for ThemeType");
        });
    }
}

impl Serialize for ThemeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ThemeType::Light => serializer.serialize_str("light"),
            ThemeType::Dark => serializer.serialize_str("dark"),
            ThemeType::Custom((name, _theme)) => {
                let mut theme_file =
                    app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "").unwrap();
                theme_file.push("themes");
                fs::create_dir_all(&theme_file).unwrap();
                theme_file.push(name);
                debug!("Path theme output {theme_file:?}");
                serializer.serialize_str(theme_file.to_str().unwrap())
            }
        }
    }
}

impl<'de> Deserialize<'de> for ThemeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ThemeTypeVisitor)
    }
}

struct ThemeTypeVisitor;
impl<'de> Visitor<'de> for ThemeTypeVisitor {
    type Value = ThemeType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("The only supported values are: light,dark or the path to a theme file (theme_name.theme.toml).")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ThemeType::from(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v.as_str())
    }
}

impl ValueEnum for ThemeType {
    fn value_variants<'a>() -> &'a [Self] {
        &[ThemeType::Light, ThemeType::Dark]
    }

    fn from_str(input: &str, _ignore_case: bool) -> Result<Self, String> {
        Ok(ThemeType::from(input))
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            ThemeType::Light => Some(clap::builder::PossibleValue::new("light")),
            ThemeType::Dark => Some(clap::builder::PossibleValue::new("dark")),
            ThemeType::Custom(_) => Some(clap::builder::PossibleValue::new("custom")),
        }
    }
}
