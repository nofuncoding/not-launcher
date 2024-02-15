use druid::{Widget, WidgetExt, WindowDesc, LocalizedString};
use druid::widget::{Button, Flex, Label};

use crate::data::{
    AppState,
    config::Config
};

pub mod theme;

pub fn main_window(config: &Config) -> WindowDesc<AppState> {
    let win = WindowDesc::new(root_widget())
        .title("Not Launcher");
    win
}

fn root_widget() -> impl Widget<AppState> {
    let text = LocalizedString::new("hello_world");
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("hi");

    Flex::column().with_child(label).with_child(button)
}