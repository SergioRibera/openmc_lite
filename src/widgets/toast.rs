use std::time::Duration;

use egui::{Color32, Margin, Response, RichText, Ui};
use egui_toast::{Toast, Toasts};

const OPENMC_TOAST: u32 = 0;

pub enum OpenMCToastKind {
    Info,
    Warn,
    Error,
    Success,
}

pub fn create_toast() -> Toasts {
    Toasts::new()
        .custom_contents(OPENMC_TOAST, custom_info_toast)
        .anchor((0., 50.))
        .direction(egui::Direction::TopDown)
        .align_to_end(false)
}

fn custom_info_toast(ui: &mut Ui, toast: &mut Toast) -> Response {
    egui::Frame::default()
        .fill(Color32::from_rgb(33, 150, 243))
        .inner_margin(Margin::same(12.0))
        .rounding(4.0)
        .show(ui, |ui| {
            if let [kind, title, desc] =
                &toast.text.clone().text().split("|").collect::<Vec<&str>>()[..3]
            {
                ui.horizontal(|ui| {
                    let _kind = OpenMCToastKind::from(*kind);
                    ui.vertical(|ui| {
                        ui.label(RichText::new(*title).color(Color32::WHITE));
                        ui.label(RichText::new(*desc).color(Color32::WHITE));
                    });
                    if ui.button("Close me").clicked() {
                        toast.close();
                    }
                });
            }
        })
        .response
}

pub fn add_toast(toasts: &mut Toasts, title: &str, desc: &str, kind: OpenMCToastKind) {
    toasts.add(Toast {
        text: format!("{}|{title}|{desc}", kind.to_string()).into(),
        kind: egui_toast::ToastKind::Custom(OPENMC_TOAST),
        options: egui_toast::ToastOptions::with_duration(Some(Duration::from_secs(10))),
    });
}

impl ToString for OpenMCToastKind {
    fn to_string(&self) -> String {
        match self {
            OpenMCToastKind::Info => "info".to_string(),
            OpenMCToastKind::Warn => "warn".to_string(),
            OpenMCToastKind::Error => "error".to_string(),
            OpenMCToastKind::Success => "success".to_string(),
        }
    }
}

impl From<&str> for OpenMCToastKind {
    fn from(v: &str) -> Self {
        match v {
            "info" => OpenMCToastKind::Info,
            "warn" => OpenMCToastKind::Warn,
            "error" => OpenMCToastKind::Error,
            _ => OpenMCToastKind::Success,
        }
    }
}

impl From<String> for OpenMCToastKind {
    fn from(v: String) -> Self {
        let v = v.as_str();
        OpenMCToastKind::from(v)
    }
}
