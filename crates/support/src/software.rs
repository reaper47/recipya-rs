use std::process::Command;

/// Verifies whether ffmpeg is installed.
pub fn is_ffmpeg_installed() -> bool {
    Command::new("ffmpeg").arg("-version").output().is_ok()
}
