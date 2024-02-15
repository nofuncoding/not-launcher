use std::default;
use std::io::{Read, Write, BufReader, BufWriter};
use std::{env, fs::{File, OpenOptions}, path::PathBuf};

use druid::{Data, Lens, Size};
use serde::{Deserialize, Serialize};

use nl_core::cache::mkdir_if_not_exists;

const APP_NAME: &str = "Not Launcher";
const CONFIG_FILENAME: &str = "config.toml";

#[derive(Clone, Debug, Data, Lens, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {

}

impl Default for Config {
    fn default() -> Self {
        Config {}
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Data, Deserialize, Serialize)]
#[derive(Default)]
pub enum MinecraftMirror {
    #[default]
    Mojang,
    BmclApi,
    Mcbbs,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Data, Deserialize, Serialize)]
#[derive(Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}