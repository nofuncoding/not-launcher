// Don't display console on windows
#![windows_subsystem = "windows"]

use data::AppState;
use druid::AppLauncher;

mod ui;
mod data;
mod cmd;
mod delegate;

use crate::data::Config;

fn main(){
    env_logger::init();

    let config = Config::load().unwrap_or_default();
    let state = AppState::default_with_config(config);

    let window = ui::main_window(&state.config);
    AppLauncher::with_window(window)
        .launch(state)
        .expect("Failed to launch app");
}