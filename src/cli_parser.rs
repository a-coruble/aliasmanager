use std::env;

const SHELL_CONFIGURATION_FILE_PATH_ENVIRONMENT_VARIABLE: &str = "SHELL_CONFIGURATION_FILE_PATH";
const HELP_ARGUMENT: &str = "--help";

pub struct CLIConfigData {
    pub alias_command: String,
    pub alias_trigger: String,
    pub shell_configuration_file_path: String,
}

pub enum CLIConfig<'a> {
    Help,
    Valid(CLIConfigData),
    Invalid(&'a str),
}

impl CLIConfig<'_> {
    pub fn new(args: &[String]) -> CLIConfig {
        match args.len() {
            2 if args[1].as_str() != HELP_ARGUMENT => {
                CLIConfig::Invalid("Unknown argument passed, see usage with 'aliasmanager --help'")
            }
            2 => CLIConfig::Help,
            3 => match CLIConfigData::new(args) {
                Ok(config) => CLIConfig::Valid(config),
                Err(error) => CLIConfig::Invalid(error),
            },
            _ => CLIConfig::Invalid(
                "Wrong number of arguments, see usage with 'aliasmanager --help'",
            ),
        }
    }
}

impl CLIConfigData {
    pub fn new(args: &[String]) -> Result<CLIConfigData, &str> {
        let alias_trigger = args[1].clone();
        let alias_command = args[2].clone();

        env::var(SHELL_CONFIGURATION_FILE_PATH_ENVIRONMENT_VARIABLE)
            .map(|path| CLIConfigData {
                alias_command,
                alias_trigger,
                shell_configuration_file_path: path,
            })
            .map_err(|_e| "Error reading SHELL_CONFIGURATION_FILE_PATH environment variable")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_configuration_with_2_arguments_not_help() {
        let args = vec![String::from("arg1"), String::from("arg2")];
        let _error_string =
            String::from("Unknown argument passed, see usage with 'aliasmanager --help'");
        assert!(matches!(
            CLIConfig::new(&args),
            CLIConfig::Invalid(_error_string)
        ))
    }

    #[test]
    fn test_cli_configuration_with_help_argument() {
        let args = vec![String::from("arg1"), String::from("--help")];
        assert!(matches!(CLIConfig::new(&args), CLIConfig::Help))
    }

    #[test]
    fn test_cli_configuration_with_wrong_number_of_arguments() {
        let args = vec![
            String::from("arg1"),
            String::from("arg2"),
            String::from("arg3"),
            String::from("arg4"),
        ];
        let _error_string =
            String::from("Wrong number of arguments, see usage with 'aliasmanager --help'");
        assert!(matches!(
            CLIConfig::new(&args),
            CLIConfig::Invalid(_error_string)
        ))
    }

    #[test]
    fn test_valid_cli_configuration_without_env_variable() {
        let args = vec![
            String::from("program_path"),
            String::from("gco"),
            String::from("git checkout"),
        ];
        let _error_string =
            String::from("Error reading SHELL_CONFIGURATION_FILE_PATH environment variable");
        assert!(matches!(
            CLIConfig::new(&args),
            CLIConfig::Invalid(_error_string)
        ));
    }

    #[test]
    fn test_valid_cli_configuration_with_env_variable() {
        env::set_var(
            SHELL_CONFIGURATION_FILE_PATH_ENVIRONMENT_VARIABLE,
            "~/.zshrc",
        );
        let args = vec![
            String::from("program_path"),
            String::from("gco"),
            String::from("git checkout"),
        ];
        let _cli_config_data = CLIConfigData {
            alias_command: String::from("git checkout"),
            alias_trigger: String::from("gco"),
            shell_configuration_file_path: String::from("~/.zshrc"),
        };
        assert!(matches!(
            CLIConfig::new(&args),
            CLIConfig::Valid(_cli_config_data)
        ));
    }
}
