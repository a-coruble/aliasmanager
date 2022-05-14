pub struct CLIConfig {
    pub alias_command: String,
    pub alias_trigger: String,
}

impl CLIConfig {
    pub fn new(args: &Vec<String>) -> Result<CLIConfig, &'static str> {
        if args.len() != 3 {
            return Err("Incorrect number of arguments passed to aliasmanager");
        };
        let alias_trigger = args[1].clone();
        let alias_command = args[2].clone();
        Ok(CLIConfig {
            alias_command,
            alias_trigger,
        })
    }
}

