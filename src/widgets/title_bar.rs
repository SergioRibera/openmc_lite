use eframe::egui;

use crate::{data::APP_NAME, resources::Icons, MainApplication};

use super::{AppComponent, IconButton};

pub struct TitleBar;

impl AppComponent for TitleBar {
    type Context = MainApplication;

    fn with_frame(ctx: &mut Self::Context, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        let icons = &ctx.resources.icons;
        title_bar_ui(ui, frame, title_bar_rect, icons);
    }
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    icons: &Icons,
) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(
        title_bar_rect,
        Id::new("title_bar"),
        Sense::click_and_drag(),
    );

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        APP_NAME,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    // User Profile
    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            ui.image(icons.app.id(ui.ctx()), (32., 32.));
            ui.vertical(|ui| {
                ui.label("Sergio Ribera");
                ui.label("OFFLINE");
            });
            ui.image(icons.expand_arrow.id(ui.ctx()), (10., 10.));
        });
    });

    // Windows Controlls
    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);

            let close_btn = ui
                .add(IconButton::new(&icons.close))
                .on_hover_text("Close Window");
            if close_btn.clicked() {
                frame.close();
            };
            if !frame.is_web() {
                if frame.info().window_info.maximized {
                    let maximized_response = ui
                    .add(IconButton::new(&icons.restore))
                        .on_hover_text("Restore window");
                    if maximized_response.clicked() {
                        frame.set_maximized(false);
                    }
                } else {
                    let maximized_response = ui
                        .add(IconButton::new(&icons.maximize))
                        .on_hover_text("Maximize window");
                    if maximized_response.clicked() {
                        frame.set_maximized(true);
                    }
                }

                let minimized_response = ui
                    .add(IconButton::new(&icons.minimize))
                    .on_hover_text("Minimize the window");
                if minimized_response.clicked() {
                    frame.set_minimized(true);
                }
            }
        });
    });
}
