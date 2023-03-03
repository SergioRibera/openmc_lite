use egui_extras::RetainedImage;

pub mod icon;

mod icon_loader;

pub use icon_loader::Icons;
// @TODO
// mod theme;

pub struct ResourceLoader {
    pub home_bg: RetainedImage,
    pub btn_bg: RetainedImage,
    // theme: Theme,
}

impl Default for ResourceLoader {
    fn default() -> Self {
        ResourceLoader {
            home_bg: RetainedImage::from_image_bytes(
                "home_background",
                include_bytes!("../../assets/bg.jpg"),
            )
            .unwrap(),
            btn_bg: RetainedImage::from_image_bytes(
                "play_button_background",
                include_bytes!("../../assets/play_btn_bg.png"),
            )
            .unwrap(),
        }
    }
}
