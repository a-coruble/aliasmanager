use std::env;

const SHELL_CONFIGURATION_FILE_PATH_ENVIRONMENT_VARIABLE: &str = "SHELL_CONFIGURATION_FILE_PATH";
const HELP_ARGUMENT: &str = "--help";

pub struct CLIConfig {
    pub alias_command: String,
    pub alias_trigger: String,
    pub shell_configuration_file_path: String,
}

pub enum CLIConfigStatus {
    HELP,
    VALID(CLIConfig),
    INVALID(String)
}

impl CLIConfigStatus {
    pub fn new(args: &Vec<String>) -> CLIConfigStatus {
        let help_string = String::from(HELP_ARGUMENT);
        match args.len() {
            2 => {
                if args[1] != help_string {
                    return CLIConfigStatus::INVALID(String::from("Unknown argument passed, see usage with 'aliasmanager --help'"))
                }
                return CLIConfigStatus::HELP;
            },
            3 => {
                return match CLIConfig::new(args) {
                    Ok(config) => CLIConfigStatus::VALID(config),
                    Err(error) => CLIConfigStatus::INVALID(error)
                };
                
            },
            _ => {
                return CLIConfigStatus::INVALID(String::from("Wrong number of arguments, see usage with 'aliasmanager --help'"))
            }
        }
    }
}

impl CLIConfig {
    pub fn new(args: &Vec<String>) -> Result<CLIConfig, String> {
        let alias_trigger = args[1].clone();
        let alias_command = args[2].clone();

        let shell_configuration_file_path =
            env::var(SHELL_CONFIGURATION_FILE_PATH_ENVIRONMENT_VARIABLE);
        let shell_configuration_file_path = match shell_configuration_file_path {
            Ok(path) => path,
            Err(_error) => {
                return Err(String::from("Error reading SHELL_CONFIGURATION_FILE_PATH environment variable"))
            }
        };

        Ok(CLIConfig {
            alias_command,
            alias_trigger,
            shell_configuration_file_path,
        })
    }
}
