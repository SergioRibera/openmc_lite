use egui_extras::RetainedImage;

pub mod icon;

mod icon_loader;

pub use icon_loader::Icons;
// @TODO
// mod theme;

pub struct ResourceLoader {
    pub home_bg: RetainedImage,
    pub btn_bg: RetainedImage,
    pub mc_btn: RetainedImage, // Minecraft
    pub ms_btn: RetainedImage, // Microsoft
    pub lc_btn: RetainedImage, // Local
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
                include_bytes!("../../assets/buttons/play_btn_bg.png"),
            )
            .unwrap(),
            mc_btn: RetainedImage::from_image_bytes(
                "play_button_background",
                include_bytes!("../../assets/buttons/mc_account_btn.png"),
            )
            .unwrap(),
            ms_btn: RetainedImage::from_image_bytes(
                "play_button_background",
                include_bytes!("../../assets/buttons/mj_account_btn.png"),
            )
            .unwrap(),
            lc_btn: RetainedImage::from_image_bytes(
                "play_button_background",
                include_bytes!("../../assets/buttons/lc_account_btn.png"),
            )
            .unwrap(),
        }
    }
}
