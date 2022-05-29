use std::env;

const SHELL_CONFIGURATION_FILE_PATH_ENVIRONMENT_VARIABLE: &str = "SHELL_CONFIGURATION_FILE_PATH";
const HELP_ARGUMENT: &str = "--help";

pub struct CLIConfigData {
    pub alias_command: String,
    pub alias_trigger: String,
    pub shell_configuration_file_path: String,
}

pub enum CLIConfig {
    Help,
    Valid(CLIConfigData),
    Invalid(String),
}

impl CLIConfig {
    pub fn new(args: &[String]) -> CLIConfig {
        let help_string = String::from(HELP_ARGUMENT);
        match args.len() {
            2 => {
                if args[1] != help_string {
                    return CLIConfig::Invalid(String::from(
                        "Unknown argument passed, see usage with 'aliasmanager --help'",
                    ));
                }
                CLIConfig::Help
            }
            3 => match CLIConfigData::new(args) {
                Ok(config) => CLIConfig::Valid(config),
                Err(error) => CLIConfig::Invalid(error),
            },
            _ => CLIConfig::Invalid(String::from(
                "Wrong number of arguments, see usage with 'aliasmanager --help'",
            )),
        }
    }
}

impl CLIConfigData {
    pub fn new(args: &[String]) -> Result<CLIConfigData, String> {
        let alias_trigger = args[1].clone();
        let alias_command = args[2].clone();

        let shell_configuration_file_path =
            env::var(SHELL_CONFIGURATION_FILE_PATH_ENVIRONMENT_VARIABLE);
        let shell_configuration_file_path = match shell_configuration_file_path {
            Ok(path) => path,
            Err(_error) => {
                String::from("Error reading SHELL_CONFIGURATION_FILE_PATH environment variable")
            }
        };

        Ok(CLIConfigData {
            alias_command,
            alias_trigger,
            shell_configuration_file_path,
        })
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
}
