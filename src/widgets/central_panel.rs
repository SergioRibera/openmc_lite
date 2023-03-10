use eframe::epaint::Shadow;
use egui::{Align, Context, Frame, Id, InnerResponse, LayerId, Layout, Margin, Stroke, Ui};

#[must_use = "You should call .show()"]
#[derive(Default)]
pub struct CentralPanel;

impl CentralPanel {
    /// Show the panel inside a [`Ui`].
    pub fn show_inside<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        self.show_inside_dyn(ui, Box::new(add_contents))
    }

    /// Show the panel inside a [`Ui`].
    fn show_inside_dyn<'c, R>(
        self,
        ui: &mut Ui,
        add_contents: Box<dyn FnOnce(&mut Ui) -> R + 'c>,
    ) -> InnerResponse<R> {
        let panel_rect = ui.available_rect_before_wrap();
        let mut panel_ui = ui.child_ui(panel_rect, Layout::top_down(Align::Min));

        let frame = Frame::central_panel(ui.style())
            .shadow(Shadow::NONE)
            .stroke(Stroke::NONE)
            .inner_margin(Margin::same(0.))
            .outer_margin(Margin::same(0.));
        frame.show(&mut panel_ui, |ui| {
            ui.expand_to_include_rect(ui.max_rect()); // Expand frame to include it all
            add_contents(ui)
        })
    }

    /// Show the panel at the top level.
    pub fn show<R>(
        self,
        ctx: &Context,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        self.show_dyn(ctx, Box::new(add_contents))
    }

    /// Show the panel at the top level.
    fn show_dyn<'c, R>(
        self,
        ctx: &Context,
        add_contents: Box<dyn FnOnce(&mut Ui) -> R + 'c>,
    ) -> InnerResponse<R> {
        let available_rect = ctx.available_rect();
        let layer_id = LayerId::background();
        let id = Id::new("central_panel");

        let clip_rect = ctx.screen_rect();
        let mut panel_ui = Ui::new(ctx.clone(), layer_id, id, available_rect, clip_rect);

        self.show_inside_dyn(&mut panel_ui, add_contents)
    }
}
