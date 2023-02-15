use std::sync::RwLock;

use iced::{Element, Renderer};
use iced_native::Widget;

use crate::MainMessage;
use once_cell::sync::Lazy;

pub static ALERTS: Lazy<RwLock<Vec<AlertWidget>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub type OnClickListener = fn();

pub struct AlertWidget {
    pub id: usize,
    pub alert_type: AlertType,
    pub title: String,
    pub content: String,
    pub on_click: Box<OnClickListener>,
    // pub on_close: Box<dyn Fn(usize) -> AlertManagerMessage + 'a>,
}

pub enum AlertType {
    Information,
    Warning,
    Error,
}

impl AlertWidget {
    pub fn new() -> Self {
        Self {
            id: 0,
            alert_type: AlertType::Information,
            title: String::new(),
            content: String::new(),
            on_click: Box::new(|| {}),
            // on_close: Box::new(|_| AlertManagerMessage::None),
        }
    }
}

pub enum AlertManagerMessage {
    Add(),
    Remove(usize),
    None,
}

pub struct AlertManager {
    timeout_secs: u64,
}

impl AlertManager {
    pub fn new(timeout_secs: u64) -> Self {
        // let toasts = toasts
        //     .iter()
        //     .enumerate()
        //     .map(|(index, toast)| {
        //         container(column![
        //             container(
        //                 row![
        //                     text(toast.title.as_str()),
        //                     horizontal_space(Length::Fill),
        //                     button("X")
        //                         .on_press((on_close)(index))
        //                         .padding(3),
        //                 ]
        //                 .align_items(Alignment::Center)
        //             )
        //             .width(Length::Fill)
        //             .padding(5)
        //             .style(
        //                 theme::Container::Custom(Box::new(toast.status))
        //             ),
        //             horizontal_rule(1),
        //             container(text(toast.body.as_str()))
        //                 .width(Length::Fill)
        //                 .padding(5)
        //                 .style(theme::Container::Box),
        //         ])
        //         .max_width(200)
        //         .into()
        //     })
        //     .collect();

        Self { timeout_secs }
    }
}

impl Widget<MainMessage, Renderer> for AlertManager {
    fn width(&self) -> iced::Length {
        iced::Length::Shrink
    }

    fn height(&self) -> iced::Length {
        iced::Length::Shrink
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        iced_native::layout::Node::new(iced::Size { width: (), height: () })
    }

    fn draw(
        &self,
        state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        todo!()
    }
}

// fn draw_element(data: &AlertWidget) -> iced::Element<'static, MainMessage> {
//     let icon = image(format!("{}/assets/icon.png", env!("CARGO_MANIFEST_DIR")))
//         .width(iced::Length::Units(40))
//         .height(iced::Length::Units(40))
//         .content_fit(iced::ContentFit::ScaleDown);
//     container(
//         Row::new()
//             .push(icon)
//             .push(
//                 Column::new()
//                     .push(text(&data.title))
//                     .push(text(&data.content))
//                     .spacing(10)
//                     .width(iced::Length::Shrink)
//                     .height(iced::Length::Shrink),
//             )
//             .width(iced::Length::Shrink)
//             .height(iced::Length::Shrink)
//             .align_items(iced::Alignment::Center)
//             .padding(Padding::new(5))
//             .spacing(10),
//     )
//     .into()
// }
//
// pub fn info(title: &str, content: &str, on_click: OnClickListener) {
//     let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
//     MANAGER.write().unwrap().push(AlertWidget {
//         alert_type: AlertType::Information,
//         id,
//         title: title.to_string(),
//         content: content.to_string(),
//         on_click: Box::new(on_click),
//     });
// }
//
// pub fn warn(title: &str, content: &str, on_click: OnClickListener) {
//     let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
//     MANAGER.write().unwrap().push(AlertWidget {
//         alert_type: AlertType::Warning,
//         id,
//         title: title.to_string(),
//         content: content.to_string(),
//         on_click: Box::new(on_click),
//     });
// }
//
// pub fn error(title: &str, content: &str, on_click: OnClickListener) {
//     let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
//     MANAGER.write().unwrap().push(AlertWidget {
//         alert_type: AlertType::Error,
//         id,
//         title: title.to_string(),
//         content: content.to_string(),
//         on_click: Box::new(on_click),
//     });
// }
//
// pub fn notifications() -> iced::Element<'static, MainMessage> {
//     let nots = MANAGER.read().unwrap();
//     let r: iced::Element<'static, MainMessage> = column(
//         nots.iter()
//             .map(draw_element)
//             .collect::<Vec<iced::Element<'static, MainMessage>>>(),
//     )
//     .align_items(iced::Alignment::End)
//     .width(iced::Length::Shrink)
//     .height(iced::Length::Shrink)
//     .spacing(10)
//     .into();
//
//     r.explain(Color::from_rgb8(255, 0, 0))
// }
//
// pub fn pop_notification() {
//     MANAGER.write().unwrap().remove(0);
// }
