use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

use config::{Config, ConfigError, File};
use directories::{BaseDirs, ProjectDirs};
use serde::Deserialize;

const RUNTIME_CONFIGURATION_FILENAME: &str = ".aliasmanagerrc.toml";
const DEFAULT_CONFIG_CONTENT: &str = "[aliasmanager]
shell_file_path = \"$HOME/.zshrc\"
";

// TODO: Move User Aliases logic away in it's own module, handling alias files
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
pub struct RuntimeConfiguration {
    pub workspaces: Option<Vec<Workspace>>,
    pub aliasmanager: Aliasmanager,
}

impl RuntimeConfiguration {
    pub fn new() -> Result<Self, ConfigError> {
        match ProjectDirs::from("dev", "acoruble", "aliasmanager") {
            None => Err(ConfigError::Message(
                "Could not determine OS default settings path".to_string(),
            )),
            Some(project_dirs) => {
                dbg!(BaseDirs::new());
                let config_dir = project_dirs.config_dir();
                let config_dir = Path::new(config_dir);
                let file_path = config_dir.join(RUNTIME_CONFIGURATION_FILENAME);
                let config_file_exists = file_path.exists();
                if !config_file_exists {
                    if !config_dir.exists() {
                        println!("> Config dir does not exist, it will now be created");
                        create_dir_all(config_dir);
                    }
                    let mut file = OpenOptions::new()
                        .create(true)
                        .read(true)
                        .write(true)
                        .open(&file_path)
                        .unwrap();
                    file.write(DEFAULT_CONFIG_CONTENT.as_bytes());
                    Config::builder()
                        .add_source(File::with_name(file_path.as_os_str().to_str().unwrap()))
                        .build()
                        .unwrap()
                        .try_deserialize()
                } else {
                    Config::builder()
                        .add_source(File::with_name(file_path.as_os_str().to_str().unwrap()))
                        .build()
                        .unwrap()
                        .try_deserialize()
                }
            }
        }
    }
}
