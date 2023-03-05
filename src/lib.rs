pub mod args;
pub mod data;
pub mod download_svc;
pub mod resources;
pub mod screens;
pub mod settings;
pub mod widgets;

#[cfg(feature = "inspect")]
use egui_inspect::EguiInspect;

use egui_toast::Toasts;
use widgets::create_toast;

#[cfg_attr(feature = "inspect", derive(EguiInspect))]
pub struct MainState {
    pub sub_title: String,
    pub create_instance: bool,
    #[cfg_attr(feature = "inspect", inspect(hide))]
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
