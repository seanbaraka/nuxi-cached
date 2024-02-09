use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;

pub fn command(path_name: &str) {
    println!("Executing the init command on {}", path_name);
    // call the npmjs registry to create the new nuxt app.
    let mut npx_command = String::from("npx nuxi init ");
    npx_command.push_str(path_name);

    // Create progress bar
    let style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("Running NPM Command...")
        .unwrap();

    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(style);

    let mut child = Command::new("npx")
        .args(["nuxi", "init", path_name])
        .spawn()
        .expect("Could not execute the given command");

    // Get exit status and handle errors
    let exit_status = child.wait().expect("Failed to wait for NPM command");

    if !exit_status.success() {
        progress_bar.finish_with_message("NPM command failed");
        panic!(
            "NPM command exited with error code {}",
            exit_status.code().unwrap()
        );
    }

    // Finalize progress bar
    progress_bar.finish_with_message("NPM command completed");
}
