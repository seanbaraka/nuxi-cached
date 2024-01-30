use std::error::Error;

mod commands;

pub struct ArgumentsConfig {
    pub command: String,
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

        Ok(ArgumentsConfig { command })
    }
}

pub fn run(config: ArgumentsConfig) -> Result<(), Box<dyn Error>> {
    // execute the passed command
    println!("Please wait. Running {}", &config.command);
    if &config.command == "new" {
        commands::new::command()
    }
    Ok(())
}
