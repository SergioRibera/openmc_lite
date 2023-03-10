use anyhow::Result;
use eframe::{
    egui::{Context, Ui},
    epaint::TextureId,
};
use egui_extras::image::FitTo;

pub struct Icon {
    pub texture: egui_extras::RetainedImage,
    pub size: FitTo,
    pub name: String,
}

impl Icon {
    pub fn from_svg(name: &str, size: FitTo) -> Result<Self> {
        let bytes = std::fs::read(format!("./assets/icons/{name}"))?;
        Ok(Self {
            size,
            name: name.to_string(),
            texture: egui_extras::RetainedImage::from_svg_bytes_with_size(name, &bytes, size)
                .unwrap(),
        })
    }

    pub fn from_image(name: &str, size: FitTo) -> Result<Self> {
        let bytes = std::fs::read(format!("./assets/{name}"))?;
        Ok(Self {
            size,
            name: name.to_string(),
            texture: egui_extras::RetainedImage::from_image_bytes(name, &bytes).unwrap(),
        })
    }

    pub fn svg_from_path(name: &str, path: &str, size: FitTo) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        Ok(Self {
            size,
            name: name.to_string(),
            texture: egui_extras::RetainedImage::from_svg_bytes_with_size(name, &bytes, size)
                .unwrap(),
        })
    }

    pub fn image_from_path(name: &str, path: &str, size: FitTo) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        Ok(Self {
            size,
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
