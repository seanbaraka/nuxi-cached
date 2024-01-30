use nuxicachedcli::{run, ArgumentsConfig};
use std::{env, process};

mod consts;

fn main() {
    let config = ArgumentsConfig::build(env::args()).unwrap_or_else(|e| {
        eprintln!("There was a problem parsing the arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    };
}
