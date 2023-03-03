pub mod args;
pub mod data;
pub mod download_svc;
pub mod resources;
pub mod screens;
pub mod settings;
pub mod widgets;

#[cfg(debug_assertions)]
pub mod stats;

use egui_toast::Toasts;
use widgets::create_toast;

pub struct MainState {
    pub sub_title: String,
    pub create_instance: bool,
    pub toasts: Toasts,
}

impl Default for MainState {
    fn default() -> Self {
        Self {
            sub_title: Default::default(),
            create_instance: false,
            toasts: create_toast(),
        }
    }
}
