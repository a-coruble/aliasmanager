use std::env;
use std::process;

mod runtime_configuration;

// TODO: refactor to use runtime configuration
// fn run(config_data: CLIConfigData) {
//     let alias_to_write = format!(
//         "alias {}=\"{}\"\n",
//         config_data.alias_trigger, config_data.alias_command
//     );

//     let _shell_configuration_file = fs::OpenOptions::new()
//         .append(true)
//         .open(&config_data.shell_configuration_file_path)
//         .unwrap_or_else(|err| {
//             eprintln!(
//                 "Error opening shell configuration file at {}.
//                 The error we got was: {}",
//                 config_data.shell_configuration_file_path, err
//             );
//             process::exit(1);
//         })
//         .write(alias_to_write.as_bytes())
//         .unwrap_or_else(|err| {
//             eprintln!(
//                 "Error appending alias to shell configuration file.
//                 The error we got was: {}",
//                 err
//             );
//             process::exit(1);
//         });

//     process::exit(0)
// }

fn main() {
    let _args: Vec<String> = env::args().collect();
    let settings = runtime_configuration::RuntimeConfiguration::new();

    match settings {
        Err(error) => {
            eprintln!("Something went wrong, here's the error: \n{}", error);
            process::exit(1);
        }
        Ok(settings_data) => {
            dbg!(settings_data);
        }
    }
}
