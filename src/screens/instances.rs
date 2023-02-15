use iced::{
    widget::{button, Column},
    Padding,
};
use iced_aw::Grid;

use crate::{
    settings::{LauncherInstance, LauncherSettings},
    MainMessage,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceEvent {
    Add,
    Show,
}

pub fn instances<'a>(conf: LauncherSettings) -> iced::Element<'a, MainMessage> {
    let mut grid = Grid::with_columns(3).push(
        button("Add New")
            .padding(Padding::new(20))
            .width(iced::Length::Fill)
            .width(iced::Length::Units(300))
            .on_press(MainMessage::InstanceView(InstanceEvent::Add)),
    );
    for instance in conf.instances.iter() {
        grid.insert(instance_element(instance));
    }

    grid.into()
    // .height(iced::Length::Shrink)
    // .width(iced::Length::Shrink)
    // .align_items(iced::Alignment::Center)
    // .spacing(10)
    // .padding(Padding::new(10))
    // .into()
}

fn instance_element<'a>(instance: &LauncherInstance) -> iced::Element<'a, MainMessage> {
    Column::new()
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .align_items(iced::Alignment::Center)
        .into()
}
