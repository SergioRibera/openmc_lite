// mod notifications;
mod title_bar;

use marcel::Theme;
// pub use notifications::*;
pub use title_bar::*;

pub type OpenMCRenderer = iced::Renderer<Theme>;
pub type Element<'a, Message> = iced::Element<'a, Message, OpenMCRenderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, OpenMCRenderer>;
pub type Button<'a, Message> = iced::widget::Button<'a, Message, OpenMCRenderer>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, OpenMCRenderer>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, OpenMCRenderer>;
pub type Text<'a> = iced::widget::Text<'a, OpenMCRenderer>;
