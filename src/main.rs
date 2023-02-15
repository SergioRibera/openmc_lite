#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use data::theme::ThemeType;
use data::APP_NAME;
use iced::{window, Application};
use iced::{window::Icon, Settings};
use marcel::Theme;
use screens::{tab_buttons, InstanceEvent, ViewType};
use settings::{LauncherInstance, LauncherSettings};
use widgets::{title_bar, Column, Container, OpenMCRenderer};

mod args;
mod data;
mod screens;
mod settings;
mod widgets;

fn main() -> iced::Result {
    let icon_bytes = include_bytes!("../assets/icon.png");

    MainApplication::run(Settings {
        default_font: Some(include_bytes!("../assets/fonts/MinecraftRegular-Bmg3.otf")),
        window: iced::window::Settings {
            decorations: false,
            min_size: Some((1080, 720)),
            size: (1080, 720),
            icon: Some(Icon::from_file_data(icon_bytes, None).unwrap()),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
struct MainApplication {
    launcher_config: LauncherSettings,
    theme: Theme,
    curr_view: ViewType,
    windows_state: WindowEvent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowEvent {
    Normal,
    Minimize,
    Maximize,
    Close,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    None,
    ThemeChanged(ThemeType),
    ViewChanged(ViewType),
    WindowEvent(WindowEvent),

    InstanceView(InstanceEvent),

    LauncherInstanceChanged(LauncherInstance),
}

impl Application for MainApplication {
    type Message = MainMessage;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let launcher_config = LauncherSettings::new();
        (
            Self {
                launcher_config: launcher_config.clone(),
                theme: launcher_config.theme.apply(),
                curr_view: if launcher_config.instances.is_empty() {
                    ViewType::Instances
                } else {
                    ViewType::Home
                },
                windows_state: WindowEvent::Normal,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        APP_NAME.to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MainMessage::ThemeChanged(theme) => {
                self.theme = theme.clone().apply();
            }
            MainMessage::ViewChanged(view) => {
                self.curr_view = view;
            }
            MainMessage::WindowEvent(e) => {
                return match e {
                    WindowEvent::Close => window::close(),
                    WindowEvent::Minimize => {
                        let r = window::minimize(self.windows_state == WindowEvent::Minimize);
                        self.windows_state = e;
                        r
                    }
                    WindowEvent::Maximize => {
                        let r = window::maximize(self.windows_state == WindowEvent::Maximize);
                        self.windows_state = e;
                        r
                    }
                    _ => iced::Command::none(),
                };
            }
            MainMessage::LauncherInstanceChanged(instance) => {
                self.launcher_config.last_launched = Some(instance);
            }
            _ => {}
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, OpenMCRenderer> {
        let variable_content = match self.curr_view {
            // ViewType::Instances => screens::instances(self.launcher_config.clone()),
            _ => screens::home(self.launcher_config.clone(), &self.theme),
        };

        // let content_overlay = FloatingElement::new(variable_content, notifications)
        //     .anchor(iced_aw::floating_element::Anchor::NorthEast)
        //     .offset(Offset { x: 20., y: 20. });

        let content = Column::new()
            .push(title_bar(&self.theme))
            .push(tab_buttons(&self.theme))
            // .push(content_overlay)
            .push(variable_content);

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(self.theme.get_container(&"default".to_string()))
            .into()
    }
}
