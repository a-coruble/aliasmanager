use config::{Config, ConfigError, File};
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./config/aliasmanager.toml";

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
    pub workspaces: Vec<Workspace>,
    pub aliasmanager: Aliasmanager,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .build()?;
        settings.try_deserialize()
    }
}
