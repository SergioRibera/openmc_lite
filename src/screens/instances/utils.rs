use egui_stylist::{StylistFileDialog, StylistState};
use log::debug;

use crate::resources::icon::Icon;

pub fn select_icon(state: &mut StylistState) -> Option<(String, Icon)> {
    if let Some(path) = state.file_dialog(StylistFileDialog::Open, Some(("", &["png"]))) {
        debug!("Selected Path where Save: {path:?}");
        let path_str = path.to_str().unwrap().to_string();
        return match Icon::image_from_path(
            path.file_name().unwrap().to_str().unwrap(),
            &path_str,
            egui_extras::image::FitTo::Original,
        ) {
            Ok(icon) => Some((path_str, icon)),
            Err(_) => None,
        };
    }
    None
}
