// Prevent console window from showing up on Windows
// #![windows_subsystem = "windows"]

use iced::Sandbox;

mod config;
mod app;

fn main() -> iced::Result {
    env_logger::init();

    app::Main::run(iced::Settings::default())
}