use std::path::PathBuf;
use std::{env, process, process::Command};

fn main() {
    if env::var("SKIP_BUILD_RS").is_ok() {
        println!("Skipping build.rs tasks");
        return;
    }

    let web_dir = PathBuf::from("../../../web/app");

    let status = Command::new("npm")
        .arg("i")
        .current_dir(&web_dir)
        .status()
        .expect("Failed to install npm packages");

    if !status.success() {
        println!("npm install failed with status: {status}");
        process::exit(1);
    }

    let status = Command::new("npm")
        .args(["run", "build"])
        .current_dir(&web_dir)
        .status()
        .expect("Failed to build the web app");

    if !status.success() {
        println!("npm build failed with status: {status}");
        process::exit(1);
    }
}
