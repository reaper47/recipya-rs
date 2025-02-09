use std::path::Path;
use std::process::Command;

use tracing::{error, info};

use crate::error::Result;

/// Generates the GitHub sponsors image for use in the README.
pub fn generate_sponsors_image() -> Result<()> {
    let web_dir = Path::new("web").join("sponsors");

    let npm_install = Command::new("npm")
        .arg("install")
        .current_dir(&web_dir)
        .status()
        .expect("Failed to run `npm install`");

    if !npm_install.success() {
        error!("`npm install` failed");
        std::process::exit(1);
    }

    let sponsorkit = Command::new("npx")
        .args(["sponsorkit", "-o", "out"])
        .current_dir(web_dir)
        .status()
        .expect("Failed to run `npx sponsorkit`");

    if !sponsorkit.success() {
        error!("npx sponsorkit` command failed");
        std::process::exit(1);
    }

    info!("Sponsor images generated successfully!");

    Ok(())
}
