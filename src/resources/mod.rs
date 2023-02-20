use eframe::egui::{Context, FontDefinitions};
use egui_extras::RetainedImage;

pub mod icon;

mod fonts;
mod icon_loader;
// @TODO
// mod theme;

pub use self::icon_loader::Icons;

pub struct ResourceLoader {
    pub icons: Icons,
    pub home_bg: RetainedImage,
    fonts: FontDefinitions,
    // theme: Theme,
}

impl ResourceLoader {
    pub fn new() -> Self {
        ResourceLoader {
            icons: Icons::preload().unwrap(),
            home_bg: RetainedImage::from_image_bytes(
                "home_background",
                include_bytes!("../../assets/bg.jpg"),
            )
            .unwrap(),
            fonts: fonts::load_fonts(),
        }
    }

    pub fn fonts(&self) -> FontDefinitions {
        self.fonts.clone()
    }
}
