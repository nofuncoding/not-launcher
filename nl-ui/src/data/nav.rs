use std::default;
use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Data, PartialEq, Eq, Hash)]
pub enum Route {
    Home,
    Settings,
    About,
}

#[derive(Default, Clone, Debug, Data, PartialEq, Eq, Deserialize, Serialize)]
pub enum Nav {
    #[default]
    Home,
    Settings,
    About,
}

impl Nav {
    pub fn route(&self) -> Route {
        match self {
            Nav::Home => Route::Home,
            Nav::Settings => Route::Settings,
            Nav::About => Route::About,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Nav::Home => "Home",
            Nav::Settings => "Settings",
            Nav::About => "About",
        }
    }
}