use std::process::Command;

use tracing::warn;

/// Verifies whether ffmpeg is installed.
pub fn is_ffmpeg_installed() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .inspect_err(|err| {
            warn!(?err, "ffmpeg check failed");
        })
        .is_ok()
}
