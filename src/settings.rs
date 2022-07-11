use std::{
    fs::{create_dir_all, OpenOptions},
    path::{Path, PathBuf},
};

use config::{Config, ConfigError, File};
use directories::ProjectDirs;
use serde::Deserialize;

const CONFIG_FILE_NAME: &str = "config.toml";
const FALLBACK_CONFIG_FILE_PATH: &str = "./config/example.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Alias {
    pub trigger: String,
    pub command: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Workspace {
    pub name: String,
    pub aliases: Vec<Alias>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Aliasmanager {
    pub shell_file_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub workspaces: Option<Vec<Workspace>>,
    pub aliasmanager: Aliasmanager,
}

// enum SettingsFileError {
//     AlreadyExists,
//     CreationFailed,
//     DoesNotExists,
// }

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // // /users/acoruble/config/dev.acoruble.aliasmanager/.config/

        let settings = Config::builder()
            .add_source(File::with_name(FALLBACK_CONFIG_FILE_PATH))
            .build()?;
        settings.try_deserialize()
    }
}
