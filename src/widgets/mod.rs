// mod notifications;
mod title_bar;
mod icon_button;

use eframe::{egui::Ui, Frame};
// pub use notifications::*;
pub use title_bar::*;
pub use icon_button::IconButton;

pub trait AppComponent {
    type Context;

    #[allow(unused)]
    fn add(ctx: &mut Self::Context, ui: &mut Ui) {}

    #[allow(unused)]
    fn with_frame(ctx: &mut Self::Context, ui: &mut Ui, frame: &mut Frame) {}
}
