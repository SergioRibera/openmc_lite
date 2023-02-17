use anyhow::Result;
use eframe::{egui::{Context, Ui, Widget}, epaint::TextureId};
use egui_extras::image::FitTo;

pub struct Icon {
    pub texture: egui_extras::RetainedImage,
    pub size: FitTo,
}

impl Icon {
    pub fn from_svg(name: &str, size: FitTo, alloc: &Context) -> Result<Self> {
        Ok(Self {
            size,
            texture: egui_extras::RetainedImage::from_svg_bytes_with_size(
                name,
                &std::fs::read(format!("./assets/icons/{name}"))?,
                size,
            ).unwrap(),
        })
    }

    pub fn from_image(name: &str, size: FitTo, alloc: &Context) -> Result<Self> {
        Ok(Self {
            size,
            texture: egui_extras::RetainedImage::from_image_bytes(
                name,
                &std::fs::read(format!("./assets/{name}"))?,
            ).unwrap(),
        })
    }

    pub fn id(&self, ctx: &Context) -> TextureId {
        self.texture.texture_id(ctx)
    }

    pub fn size(&self) -> eframe::epaint::Vec2 {
        self.texture.size_vec2()
    }

    pub fn show(&self, ui: &mut Ui) {
        self.texture.show(ui);
    }
}

impl Widget for Icon {
    fn ui(self, ui: &mut Ui) -> eframe::egui::Response {
        self.texture.show(ui)
    }
}

impl Widget for &Icon {
    fn ui(self, ui: &mut Ui) -> eframe::egui::Response {
        self.texture.show(ui)
    }
}
