
use iced::{Sandbox, Element, Length, Alignment, Theme};
use iced::widget::{
    text, container, column, pick_list,
};

use crate::config::Config;

pub struct Main {
    config: Config,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(Theme),
}

impl Sandbox for Main {
    type Message = Message;

    fn new() -> Self {
        let config = Config::load().unwrap_or_default();
        
        Self {
            config: config.clone(),
            theme: config.theme(),
        }
    }

    fn title(&self) -> String {
        "Not Launcher".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.config.set_theme(&theme);
                self.config.save();
                self.theme = theme;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text("Hello, world!"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
                .width(Length::Fill),
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}