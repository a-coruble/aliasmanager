use std::env;
use std::process;

mod cli_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cli_parser::CLIConfig::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("{} {}", config.alias_command, config.alias_trigger);
}
