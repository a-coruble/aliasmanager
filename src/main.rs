use std::env;
use std::fs;
use std::io::Write;
use std::process;

mod cli_parser;

const ZSHRC_FILE_PATH: &str = "/Users/acoruble/.zshrc";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cli_parser::CLIConfig::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let mut shell_configuration_file = fs::OpenOptions::new()
        .append(true)
        .open(ZSHRC_FILE_PATH)
        .unwrap_or_else(|err| {
            eprintln!(
                "Error opening shell configuration file at {}.
                The error we got was: {}",
                ZSHRC_FILE_PATH, err
            );
            process::exit(1);
        });

    shell_configuration_file
        .write(
            format!(
                "alias {}=\"{}\"\n",
                config.alias_trigger, config.alias_command
            )
            .as_bytes(),
        )
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
