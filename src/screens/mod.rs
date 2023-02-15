mod home;
mod instances;

use iced::{
    widget::{button, row},
    Padding,
};
pub use home::home;
pub use instances::{instances, InstanceEvent};
use marcel::Theme;

use crate::MainMessage;

#[derive(Debug, Clone, Copy)]
pub enum ViewType {
    Home,
    Account,
    Instances,
    Preferences,
}

pub fn tab_buttons<'a>(theme: &Theme) -> iced::Element<'a, MainMessage> {
    let play_btn = button("Play")
        .on_press(MainMessage::ViewChanged(ViewType::Home))
        .style(theme.button.get("default-tab").unwrap().clone())
        .padding(Padding::new(5));
    let instances_btn = button("Instances")
        .on_press(MainMessage::ViewChanged(ViewType::Instances))
        .style(theme.button.get("default-tab").unwrap().clone())
        .padding(Padding::new(5));
    let prefs_btn = button("Preferences")
        .on_press(MainMessage::ViewChanged(ViewType::Preferences))
        .style(theme.button.get("default-tab").unwrap().clone())
        .padding(Padding::new(5));

    row![play_btn, instances_btn, prefs_btn]
        .height(iced::Length::Shrink)
        .width(iced::Length::Fill)
        .spacing(10)
        .padding(Padding::new(10))
        .into()
}
