use std::error::Error;

mod commands;

pub struct ArgumentsConfig {
    pub command: String,
    pub query: String,
}

impl ArgumentsConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<ArgumentsConfig, &'static str> {
        // Lets get rid of the first flag passed
        args.next();

        // The command should generally be the second argument
        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("You need to pass down the command"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("You need to pass down the name of the application"),
        };

        Ok(ArgumentsConfig { command, query })
    }
}

pub fn run(config: ArgumentsConfig) -> Result<(), Box<dyn Error>> {
    // execute the passed command
    println!(
        "Please wait. Initializing a new nuxt application at '{}'",
        &config.query
    );
    if &config.command == "init" {
        commands::init::command(&config.query)
    }
    Ok(())
}
