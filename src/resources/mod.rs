use egui_extras::RetainedImage;

pub mod icon;

mod icon_loader;
// @TODO
// mod theme;

pub use self::icon_loader::Icons;

pub struct ResourceLoader {
    pub icons: Icons,
    pub home_bg: RetainedImage,
    pub btn_bg: RetainedImage,
    // theme: Theme,
}

impl Default for ResourceLoader {
    fn default() -> Self {
        ResourceLoader {
            icons: Icons::preload().unwrap(),
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
