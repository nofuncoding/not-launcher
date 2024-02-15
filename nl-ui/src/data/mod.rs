pub mod config;
mod nav;

use std::{
    mem,
};

use druid::{
    im::{Vector},
    Data, Lens
};

pub use crate::data::{
    config::Config,
    nav::Nav,
};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub nav: Nav,
    pub history: Vector<Nav>,
    pub config: Config,
}

impl AppState {
    pub fn default_with_config(config: Config) -> Self {
        AppState {
            nav: Nav::default(),
            history: Vector::new(),
            config,
        }
    }
}

// Navigation
impl AppState {
    pub fn navigate(&mut self, nav: &Nav) {
        if &self.nav != nav {
            let prev: Nav = mem::replace(&mut self.nav, nav.to_owned());
            self.history.push_back(prev);
        }
    }

    pub fn back(&mut self) {
        if let Some(prev) = self.history.pop_back() {
            self.nav = prev;
        }
    }

    pub fn refresh(&mut self) {
        let current: Nav = mem::replace(&mut self.nav, Nav::default());
        self.nav = current;
    }
}