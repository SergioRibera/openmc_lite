use super::icon::Icon;
use anyhow::Result;
use egui_extras::image::FitTo;

pub struct Icons {
    pub app: Icon,
    pub expand_arrow: Icon,
    pub minimize: Icon,
    pub maximize: Icon,
    pub restore: Icon,
    pub close: Icon,
    pub light_mode: Icon,
    pub night_mode: Icon,
}

impl Icons {
    pub fn preload() -> Result<Self> {
        let this = Self {
            app: Icon::from_image("app.png", FitTo::Size(24, 24))?,
            expand_arrow: Icon::from_svg("expand_arrow.svg", FitTo::Size(24, 24))?,
            close: Icon::from_svg("close.svg", FitTo::Size(24, 24))?,
            minimize: Icon::from_svg("minus.svg", FitTo::Size(24, 24))?,
            maximize: Icon::from_svg("maximize.svg", FitTo::Size(24, 24))?,
            restore: Icon::from_svg("restore.svg", FitTo::Size(24, 24))?,
            light_mode: Icon::from_svg("light-mode.svg", FitTo::Size(24, 24))?,
            night_mode: Icon::from_svg("night-mode.svg", FitTo::Size(24, 24))?,
        };

        Ok(this)
    }
}
