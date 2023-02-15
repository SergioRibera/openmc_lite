use iced::Padding;
use iced::widget::image;
use marcel::Theme;

use crate::widgets::{Button, Container, Element, Row, Text};
use crate::{data::APP_NAME, MainMessage, WindowEvent};

pub fn title_bar<'a>(theme: &Theme) -> Element<'a, MainMessage> {
    Row::new()
        // .push(user_profile())
        .push(app_title())
        .push(controlls(theme))
        .padding(Padding::new(5))
        .width(iced::Length::Fill)
        .height(iced::Length::Units(38))
        .into()
}

pub fn user_profile<'a>() -> Element<'a, MainMessage> {
    Row::new()
        // .push(app_icon())
        .push(Text::from(APP_NAME.to_string()))
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .align_items(iced::Alignment::Center)
        .spacing(5)
        .into()
}

pub fn app_title<'a>() -> Element<'a, MainMessage> {
    Row::new()
        // .push(app_icon())
        .push(Text::from(APP_NAME.to_string()).width(iced::Length::Shrink))
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .align_items(iced::Alignment::Center)
        .spacing(5)
        .into()
}

pub fn controlls<'a>(theme: &Theme) -> Element<'a, MainMessage> {
    Row::new()
        .push(
            Button::new("x")
                .style(theme.button.get("window-action").unwrap().clone())
                .on_press(MainMessage::WindowEvent(WindowEvent::Close)),
        )
        .width(iced::Length::Shrink)
        .spacing(5)
        .into()
}

// pub fn app_icon<'a>() -> Container<'a, MainMessage> {
//     Container::new(
//         image(format!("{}/assets/icon.png", env!("CARGO_MANIFEST_DIR")))
//             .width(iced::Length::Units(18)),
//     )
//     .width(iced::Length::Shrink)
//     .center_x()
// }
