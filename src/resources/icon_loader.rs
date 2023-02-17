use super::icon::Icon;
use anyhow::Result;
use eframe::egui::Context;
use egui_extras::image::FitTo;

pub struct Icons {
    pub app: Icon,
    pub expand_arrow: Icon,
    pub minimize: Icon,
    pub maximize: Icon,
    pub restore: Icon,
    pub close: Icon,
}

impl Icons {
    pub fn preload(alloc: &Context) -> Result<Self> {
        let this = Self {
            app: Icon::from_image("app.png", FitTo::Size(24, 24), alloc)?,
            expand_arrow: Icon::from_svg("expand_arrow.svg", FitTo::Size(24, 24), alloc)?,
            close: Icon::from_svg("close.svg", FitTo::Size(24, 24), alloc)?,
            minimize: Icon::from_svg("minus.svg", FitTo::Size(24, 24), alloc)?,
            maximize: Icon::from_svg("maximize.svg", FitTo::Size(24, 24), alloc)?,
            restore: Icon::from_svg("restore.svg", FitTo::Size(24, 24), alloc)?,
        };

        Ok(this)
    }
}
