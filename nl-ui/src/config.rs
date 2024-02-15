use std::default;
use std::io::{Read, Write, BufReader, BufWriter};
use std::{env, fs::{File, OpenOptions}, path::PathBuf};

use serde::{Deserialize, Serialize};

use nl_core::cache::mkdir_if_not_exists;

const APP_NAME: &str = "Not Launcher";
const CONFIG_FILENAME: &str = "config.toml";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: iced::Theme::Light.to_string(),
        }
    }
}

impl Config {
    pub fn base_directory() -> PathBuf {
        PathBuf::from(APP_NAME)
    }

    pub fn cache_directory() -> PathBuf {
        Self::base_directory().join("cache")
    }

    fn config_path() -> PathBuf {
        Self::base_directory().join(CONFIG_FILENAME)
    }
}

impl Config {
    pub fn load() -> Option<Config> {
        let path = Self::config_path();
        if let Ok(file) = File::open(&path) {
            log::info!("Loading config: {:?}", &path);
            let mut reader = BufReader::new(file);
            let mut str = String::new();
            reader.read_to_string(&mut str).expect("Failed to read config");
            Some(toml::from_str(&str).expect("Failed to parse config"))
        } else {
            None
        }
    }

    pub fn save(&self) {
        let dir = Self::base_directory();
        let path = Self::config_path();
        mkdir_if_not_exists(&dir);

        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);
        #[cfg(target_family = "unix")]
        options.mode(0o600);

        let file = options.open(&path).expect("Failed to open config");
        let mut writer = BufWriter::new(file);

        writer.write_all(
            toml::to_string_pretty(self).expect("Failed to serialize config").as_bytes()
        ).expect("Failed to write config");
        log::info!("Config saved to {:?}", &path);
    }
}

impl Config {
    pub fn theme(&self) -> iced::Theme {
        let theme = self.theme.as_str();
        match theme {
            "Light" => iced::Theme::Light,
            "Dark" => iced::Theme::Dark,
            "Dracula" => iced::Theme::Dracula,
            "Nord" => iced::Theme::Nord,
            "Solarized Light" => iced::Theme::SolarizedLight,
            "Solarized Dark" => iced::Theme::SolarizedDark,
            "Gruvbox Light" => iced::Theme::GruvboxLight,
            "Gruvbox Dark" => iced::Theme::GruvboxDark,
            "Catppuccin Latte" => iced::Theme::CatppuccinLatte,
            "Catppuccin Frappé" => iced::Theme::CatppuccinFrappe,
            "Catppuccin Macchiato" => iced::Theme::CatppuccinMacchiato,
            "Catppuccin Mocha" => iced::Theme::CatppuccinMocha,
            "Tokyo Night" => iced::Theme::TokyoNight,
            "Tokyo Night Storm" => iced::Theme::TokyoNightStorm,
            "Tokyo Night Light" => iced::Theme::TokyoNightLight,
            "Kanagawa Wave" => iced::Theme::KanagawaWave,
            "Kanagawa Dragon" => iced::Theme::KanagawaDragon,
            "Kanagawa Lotus" => iced::Theme::KanagawaLotus,
            "Moonfly" => iced::Theme::Moonfly,
            "Nightfly" => iced::Theme::Nightfly,
            "Oxocarbon" => iced::Theme::Oxocarbon,
            _ => iced::Theme::Light,
        }
    }

    pub fn set_theme(&mut self, theme: &iced::Theme) {
        self.theme = theme.to_string();
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[derive(Default)]
pub enum MinecraftMirror {
    #[default]
    Mojang,
    BmclApi,
    Mcbbs,
}