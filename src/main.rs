use std::env;
use std::fs;
use std::io::Write;
use std::process;

mod cli_parser;
use cli_parser::CLIConfigStatus;
use cli_parser::CLIConfig;

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

fn run(config: CLIConfig) {
    let alias_to_write = format!(
        "alias {}=\"{}\"\n",
        config.alias_trigger, config.alias_command
    );

    let _shell_configuration_file = fs::OpenOptions::new()
        .append(true)
        .open(&config.shell_configuration_file_path)
        .unwrap_or_else(|err| {
            eprintln!(
                "Error opening shell configuration file at {}.
                The error we got was: {}",
                config.shell_configuration_file_path, err
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_status = CLIConfigStatus::new(&args);

    match config_status {
        CLIConfigStatus::VALID(config) => {
            run(config);
        },
        CLIConfigStatus::INVALID(error) => {
            eprintln!("Something went wrong, here is the error:\n{}", error);
            process::exit(1);
        },
        CLIConfigStatus::HELP => {
            println!("{}", HELP_MESSAGE);
            process::exit(0);
        }
    };
}
