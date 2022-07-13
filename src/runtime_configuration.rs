use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

use config::{Config, ConfigError, File};
use directories::ProjectDirs;
use serde::Deserialize;

const RUNTIME_CONFIGURATION_FILENAME: &str = ".aliasmanager_rc.toml";
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

fn create_new_file(path: &Path, permissions: &str) -> Result<std::fs::File, String> {
    if permissions.len() != 2 {
        Err("Wrong parameter for create_new_file".to_string())
    } else {
        let mut opener = OpenOptions::new();
        if !path.exists() {
            opener.create(true);
        }
        for c in permissions.chars() {
            match c {
                'r' => opener.read(true),
                'w' => opener.write(true),
                _ => {
                    return Err("Wrong permissions string parameter for create_new_file".to_string())
                }
            };
        }
        opener
            .open(&path)
            .map_err(|err| format!("Could not create file with error message:\n {}", err))
    }
}

impl RuntimeConfiguration {
    pub fn new() -> Result<Self, ConfigError> {
        match ProjectDirs::from("dev", "acoruble", "aliasmanager") {
            None => Err(ConfigError::Message(
                "Could not determine OS default settings path".to_string(),
            )),
            Some(project_dirs) => {
                let config_dir = Path::new(project_dirs.config_dir());
                let file_path = config_dir.join(RUNTIME_CONFIGURATION_FILENAME);
                if !file_path.exists() {
                    if !config_dir.exists() {
                        println!("> Config dir does not exist, it will now be created");
                        create_dir_all(config_dir);
                    }
                    create_new_file(&file_path, "rw")
                        .unwrap()
                        .write(DEFAULT_CONFIG_CONTENT.as_bytes());
                }
                Config::builder()
                    .add_source(File::with_name(file_path.as_os_str().to_str().unwrap()))
                    .build()
                    .unwrap()
                    .try_deserialize()
            }
        }
    }
}
