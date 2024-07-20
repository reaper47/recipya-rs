use std::process::Command;

fn main() {
    if cfg!(feature = "docs") {
        docs()
    }
}

fn docs() {
    let status = Command::new("hugo")
        .args(["-s", "docs", "--gc", "--minify"])
        .status()
        .expect("Failed to execute Hugo command");

    if !status.success() {
        panic!("Hugo command failed with status {:?}", status);
    }
}
