mod central_panel;
mod covered_image;
mod grid_wrapped;
mod icon_button;
mod image_button;
mod modal;
mod progress_button;
mod steps;
mod tabs;
mod title_bar;
mod toast;

use crate::data::APP_INFO;
use egui::Vec2;
use egui_stylist::StylistFileDialog;
use log::debug;
use std::path::PathBuf;

pub use central_panel::*;
pub use covered_image::*;
pub use grid_wrapped::*;
pub use icon_button::IconButton;
pub use image_button::*;
pub use modal::*;
pub use progress_button::*;
pub use steps::*;
pub use tabs::*;
pub use title_bar::*;
pub use toast::*;

pub fn open_file_dialog(
    kind: StylistFileDialog,
    filter: Option<(&str, &[&str])>,
) -> Option<PathBuf> {
    // Option a popup to save the file to a given directory
    let mut theme_folder =
        app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "").unwrap();
    theme_folder.push("themes");

    debug!("Open File Dialog Directory Path: {theme_folder:?}");
    match kind {
        StylistFileDialog::Open => {
            let mut builder = rfd::FileDialog::new();
            if let Some(filter) = filter {
                builder = builder.add_filter(filter.0, filter.1)
            }
            builder.set_directory(&theme_folder).pick_file()
        }
        StylistFileDialog::Save => {
            let mut builder = rfd::FileDialog::new();
            if let Some(filter) = filter {
                builder = builder
                    .add_filter(filter.0, filter.1)
                    .set_file_name("MyTheme.theme.toml")
            }
            builder.set_directory(&theme_folder).save_file()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CoveredRatioType {
    Cover,
    Container,
}

pub fn calculate_ratio_size(
    object_size: Vec2,
    container_size: Vec2,
    ratio_type: CoveredRatioType,
) -> Vec2 {
    let w_ratio = container_size.x / object_size.x;
    let h_ratio = container_size.y / object_size.y;

    let ratio = match ratio_type {
        CoveredRatioType::Cover => w_ratio.max(h_ratio),
        CoveredRatioType::Container => w_ratio.min(h_ratio),
    };

    Vec2::new(object_size.x * ratio, object_size.y * ratio)
}
