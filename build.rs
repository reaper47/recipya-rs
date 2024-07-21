use std::process::Command;

fn main() {
    if cfg!(feature = "docs") {
        docs()
    }

    if cfg!(feature = "web") {
        web()
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

fn web() {
    let status = Command::new("npm")
        .args(["--prefix", "./web/app", "install", "./web/app"])
        .status()
        .expect("Failed to execute npm command");

    if !status.success() {
        panic!("npm install command failed with status {:?}", status);
    }

    let status = Command::new("npm")
        .args(["--prefix", "./web/app", "run", "build"])
        .status()
        .expect("Failed to execute npm command");

    if !status.success() {
        panic!("npm run command failed with status {:?}", status);
    }
}
