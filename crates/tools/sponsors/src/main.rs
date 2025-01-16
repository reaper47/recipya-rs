use std::path::Path;
use std::process::Command;

fn main() {
    let web_dir = Path::new("web").join("sponsors");

    let npm_install = Command::new("npm")
        .arg("install")
        .current_dir(&web_dir)
        .status()
        .expect("Failed to run `npm install`");

    if !npm_install.success() {
        println!("`npm install` failed");
        std::process::exit(1);
    }

    let sponsorkit = Command::new("npx")
        .arg("sponsorkit")
        .arg("-o")
        .arg(&web_dir.join("sponsorkit.svg"))
        .current_dir(&web_dir)
        .status()
        .expect("Failed to run `npx sponsorkit`");

    if !sponsorkit.success() {
        println!("npx sponsorkit` command failed");
        std::process::exit(1);
    }

    println!("Sponsor images generated successfully!");
}
