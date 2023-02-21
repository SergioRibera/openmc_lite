mod grid_wrapped;
mod icon_button;
mod title_bar;
mod toast;

use crate::data::APP_INFO;
use eframe::{egui::Ui, Frame};
use egui_stylist::StylistFileDialog;
use log::debug;
use std::path::PathBuf;

pub use grid_wrapped::*;
pub use icon_button::IconButton;
pub use title_bar::*;
pub use toast::*;

pub trait AppComponent {
    type Context;

    #[allow(unused)]
    fn add(ctx: &mut Self::Context, ui: &mut Ui) {}

    #[allow(unused)]
    fn with_frame(ctx: &mut Self::Context, ui: &mut Ui, frame: &mut Frame) {}
}

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
                builder = builder.add_filter(filter.0, filter.1)
            }
            builder.set_directory(&theme_folder).save_file()
        }
    }
}
