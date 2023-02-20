use anyhow::Result;
use eframe::{
    egui::{Context, Ui, Widget},
    epaint::TextureId,
};
use egui_extras::image::FitTo;

pub struct Icon {
    pub texture: egui_extras::RetainedImage,
    pub size: FitTo,
    name: String,
    bytes: Vec<u8>,
    is_svg: bool,
}

impl Icon {
    pub fn from_svg(name: &str, size: FitTo) -> Result<Self> {
        let bytes = std::fs::read(format!("./assets/icons/{name}"))?;
        Ok(Self {
            size,
            is_svg: true,
            bytes: bytes.clone(),
            name: name.to_string(),
            texture: egui_extras::RetainedImage::from_svg_bytes_with_size(name, &bytes, size)
                .unwrap(),
        })
    }

    pub fn from_image(name: &str, size: FitTo) -> Result<Self> {
        let bytes = std::fs::read(format!("./assets/{name}"))?;
        Ok(Self {
            size,
            is_svg: false,
            bytes: bytes.clone(),
            name: name.to_string(),
            texture: egui_extras::RetainedImage::from_image_bytes(name, &bytes).unwrap(),
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

impl Clone for Icon {
    fn clone(&self) -> Self {
        Self {
            texture: if self.is_svg {
                egui_extras::RetainedImage::from_svg_bytes_with_size(
                    &self.name,
                    &self.bytes,
                    self.size.clone(),
                )
                .unwrap()
            } else {
                egui_extras::RetainedImage::from_image_bytes(&self.name, &self.bytes).unwrap()
            },
            name: self.name.clone(),
            is_svg: self.is_svg,
            bytes: self.bytes.clone(),
            size: self.size.clone(),
        }
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
