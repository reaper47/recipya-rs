use tracing::warn;

use crate::Result;

pub fn init_sandbox() {
    cfg_select! {
        target_os = "linux" => {
            sandbox_helper().unwrap();
        }
        _ => {
            info!("Landlock sandboxing is not supported on this platform")
        }
    }
}

#[cfg(target_os = "linux")]
fn sandbox_helper() -> Result<()> {
    use landlock::{
        ABI, Access, AccessFs, PathBeneath, PathFd, Ruleset, RulesetAttr, RulesetCreatedAttr,
    };
    use procfs::sys::kernel::Version;
    use support::fs::get_base_dir;
    use tracing::info;

    let version = Version::current().unwrap();

    let abi = match (version.major, version.minor) {
        (7, 1..) => ABI::V9,
        (7, 0) => ABI::V8,
        (6, 15..) => ABI::V7,
        (6, 12..) => ABI::V6,
        (6, 10..) => ABI::V5,
        (6, 7..) => ABI::V4,
        (6, 2..) => ABI::V3,
        (5, 19..) => ABI::V2,
        (5, 13..) => ABI::V1,
        _ => {
            warn!("Kernel {version:?} does not support the required Landlock ABI");
            return Ok(());
        }
    };

    let read_only = AccessFs::from_read(abi);
    let read_write = AccessFs::from_all(abi);
    let read_exec = read_only | AccessFs::Execute;

    Ruleset::default()
        .handle_access(read_write)?
        .create()?
        .add_rule(PathBeneath::new(PathFd::new("./")?, read_only))?
        .add_rule(PathBeneath::new(PathFd::new("/dev")?, read_only))?
        .add_rule(PathBeneath::new(PathFd::new("/etc")?, read_only))?
        .add_rule(PathBeneath::new(PathFd::new("/lib64")?, read_only))?
        .add_rule(PathBeneath::new(PathFd::new("/tmp")?, read_only))?
        .add_rule(PathBeneath::new(PathFd::new("/usr/bin")?, read_exec))?
        .add_rule(PathBeneath::new(PathFd::new(get_base_dir()?)?, read_write))?
        .restrict_self()?;

    info!("Sandboxing enabled with Landlock ABI {abi:?}");
    Ok(())
}
