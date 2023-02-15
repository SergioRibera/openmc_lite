use iced::Padding;
use iced_aw::FloatingElement;
use marcel::Theme;

use crate::{
    settings::LauncherSettings,
    widgets::{Button, Column, Container, Element, Row, Text},
    MainMessage,
};

pub fn home<'a>(conf: LauncherSettings, theme: &Theme) -> Element<'a, MainMessage> {
    let floating = FloatingElement::new(background(theme), move || {
        let conf = conf.clone();
        Row::new()
            // .push(pick_list(
            //     conf.instances,
            //     conf.last_launched,
            //     MainMessage::LauncherInstanceChanged,
            // ))
            .push(
                Container::new(
                    Button::new(
                        Text::from("Run Instance")
                            .size(40)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    )
                    .width(iced::Length::Units(300))
                    .height(iced::Length::Units(60))
                    .style(theme.get_button(&"default".to_string()))
                    .padding(Padding::new(5)),
                )
                .width(iced::Length::Fill)
                .style(theme.get_container(&"default".to_string()))
                .center_x(),
            )
            .height(iced::Length::Shrink)
            .width(iced::Length::Shrink)
            .align_items(iced::Alignment::Center)
            .spacing(10)
            .padding(Padding::new(10))
            .into()
    })
    .anchor(iced_aw::floating_element::Anchor::SouthWest);
    Column::new()
        .push(floating)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .align_items(iced::Alignment::Center)
        .into()
}

pub fn background<'a>(theme: &Theme) -> Element<'a, MainMessage> {
    Container::new(
        image(format!("{}/assets/bg.jpg", env!("CARGO_MANIFEST_DIR")))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .content_fit(iced::ContentFit::Cover),
    )
    .width(iced::Length::Fill)
    .style(theme.get_container(&"default".to_string()))
    .center_x()
}
