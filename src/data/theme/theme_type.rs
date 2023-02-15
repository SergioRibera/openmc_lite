use std::fs;

use clap::ValueEnum;
use marcel::{serial::Theme as SerializeTheme, Theme};
use serde::{de::Visitor, Deserialize, Serialize};

use crate::data::{APP_FOLDER, APP_INFO};

#[derive(Debug, Default, Clone)]
pub enum ThemeType {
    #[default]
    Light,
    Dark,
    Custom(Theme),
}

impl From<&str> for ThemeType {
    fn from(v: &str) -> Self {
        let value = v.to_lowercase();
        let value = value.as_str();
        match value {
            "light" => ThemeType::Light,
            "dark" => ThemeType::Dark,
            path_str => {
                let content = fs::read_to_string(path_str).unwrap();
                let serialize_theme = toml::from_str::<SerializeTheme>(&&content).unwrap();
                let theme = Theme::parse(&serialize_theme.clone()).unwrap();
                ThemeType::Custom(theme)
            }
        }
    }
}

impl ThemeType {
    pub fn apply(&self) -> Theme {
        match self {
            ThemeType::Light => super::LIGHT.to_owned(),
            ThemeType::Dark => super::DARK.to_owned(),
            ThemeType::Custom(theme) => theme.clone(),
        }
    }
}

impl iced::application::StyleSheet for ThemeType {
    type Style = ThemeType;

    fn appearance(&self, style: &Self::Style) -> iced::application::Appearance {
        match style {
            ThemeType::Light => iced::application::Appearance {
                background_color: super::LIGHT.application.background_color.into(),
                text_color: super::LIGHT.application.text_color.into(),
            },
            ThemeType::Dark => iced::application::Appearance {
                background_color: super::DARK.application.background_color.into(),
                text_color: super::DARK.application.text_color.into(),
            },
            ThemeType::Custom(theme) => iced::application::Appearance {
                background_color: theme.application.background_color.into(),
                text_color: theme.application.text_color.into(),
            },
        }
    }
}

impl Serialize for ThemeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ThemeType::Light => serializer.serialize_str("light"),
            ThemeType::Dark => serializer.serialize_str("light"),
            ThemeType::Custom(theme) => {
                let theme_name = theme.name.to_lowercase().replace(' ', "");
                let mut theme_file =
                    app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, APP_FOLDER)
                        .unwrap();
                theme_file.push("themes");
                fs::create_dir_all(&theme_file).unwrap();
                theme_file.push(&format!("{theme_name}.theme.toml"));

                let content = toml::to_string(&SerializeTheme::from(theme.clone())).unwrap();
                fs::write(&theme_file, content).unwrap();
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
