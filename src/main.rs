use std::env;
use std::fs;
use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process;

use config::{Config, ConfigError, File};

mod cli_parser;
mod settings;
use cli_parser::CLIConfig;
use cli_parser::CLIConfigData;
use config::ConfigBuilder;
use directories::ProjectDirs;

const HELP_MESSAGE: &str = "
AliasManager
    A tiny utilitary CLI tool to quickly create new aliases in your shell config file.
    Please note that this programs uses the `SHELL_CONFIGURATION_FILE_PATH` environment
    variable to determine where to write the alias.

USAGE:
    aliasmanager [--OPTIONS] <trigger> <command>
                
OPTIONS:
    --help: prints this help message
";

const DEFAULT_CONFIG_CONTENT: &str = "[aliasmanager]
shell_file_path = \"$HOME/.zshrc\"
";

type ResultIO<T> = Result<T, std::io::Error>;

fn run(config_data: CLIConfigData) {
    let alias_to_write = format!(
        "alias {}=\"{}\"\n",
        config_data.alias_trigger, config_data.alias_command
    );

    let _shell_configuration_file = fs::OpenOptions::new()
        .append(true)
        .open(&config_data.shell_configuration_file_path)
        .unwrap_or_else(|err| {
            eprintln!(
                "Error opening shell configuration file at {}.
                The error we got was: {}",
                config_data.shell_configuration_file_path, err
            );
            process::exit(1);
        })
        .write(alias_to_write.as_bytes())
        .unwrap_or_else(|err| {
            eprintln!(
                "Error appending alias to shell configuration file.
                The error we got was: {}",
                err
            );
            process::exit(1);
        });

    process::exit(0)
}

fn create_default_settings_file(path: &Path) -> ResultIO<std::fs::File> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)
}

fn get_or_create_settings_file() -> ResultIO<std::fs::File> {
    match ProjectDirs::from("dev", "acoruble", "aliasmanager") {
        None => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not create directory structure",
        )),
        Some(project_dirs) => {
            let config_dir = project_dirs.config_dir();
            let config_dir = Path::new(config_dir);
            let file_path = config_dir.join("config.toml");
            // TODO Debug why this statement returns false when the file indeed exists on the fs
            let config_file_exists = file_path.join("config.toml").exists();
            dbg!(config_file_exists);
            dbg!(&file_path);
            if !config_file_exists {
                create_dir_all(config_dir);
                let mut file = create_default_settings_file(&file_path).unwrap();
                dbg!(&file);
                file.write(DEFAULT_CONFIG_CONTENT.as_bytes());
                dbg!(&file);
                dbg!(&file_path.as_os_str().to_str().unwrap());
                let settings = Config::builder()
                    .add_source(File::with_name(file_path.as_os_str().to_str().unwrap()))
                    .build();
                dbg!(settings.unwrap());

                return Ok(file);
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Could not create directory structure",
                ))
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = CLIConfig::new(&args);
    let settings = settings::Settings::new();
    let file_result = get_or_create_settings_file();

    match config {
        CLIConfig::Valid(config_data) => {
            run(config_data);
        }
        CLIConfig::Invalid(error) => {
            eprintln!("Something went wrong, here is the error:\n{}", error);
            process::exit(1);
        }
        CLIConfig::Help => {
            println!("{}", HELP_MESSAGE);
            process::exit(0);
        }
    };
}
