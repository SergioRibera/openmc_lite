use egui_stylist::{StylistFileDialog, StylistState};
use log::debug;
use mc_bootstrap::ClientBootstrap;

use crate::{
    data::data_path,
    resources::icon::Icon,
    settings::{LauncherInstance, LauncherSettings},
};

pub fn launch_instance(instance: &LauncherInstance, cfg: &LauncherSettings) {
    let v = instance.version.clone().unwrap();
    ClientBootstrap::new(
        &cfg.session.access_token,
        data_path("").to_str().unwrap(),
        &instance.java_path,
        &cfg.session.name,
        &cfg.session.uuid,
        &v.get_version_id(),
        &v.get_version_type(),
    )
    .launch()
    .unwrap();
}

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
