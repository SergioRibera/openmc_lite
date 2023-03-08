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
use widgets::{create_toast, Modal, ModalBuilder};

#[macro_use]
extern crate litcrypt;

use_litcrypt!();

pub static MODAL_ID: &'static str = "__openmc__modal";

#[cfg_attr(feature = "inspect", derive(EguiInspect))]
pub struct MainState {
    pub sub_title: String,
    pub create_instance: bool,
    #[cfg_attr(feature = "inspect", inspect(hide))]
    pub toasts: Toasts,
    #[cfg_attr(feature = "inspect", inspect(hide))]
    pub modal: Modal,
}

impl Default for MainState {
    fn default() -> Self {
        Self {
            sub_title: Default::default(),
            create_instance: false,
            toasts: create_toast(),
            modal: ModalBuilder::default()
                .set_id(MODAL_ID)
                .set_size_percent((0.6, 0.6))
                .build(),
        }
    }
}
