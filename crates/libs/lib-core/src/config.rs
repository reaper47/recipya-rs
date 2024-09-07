use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    sync::OnceLock,
};

use lib_utils::envs::get_env;
use tracing::info;

pub fn config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("Fatal - Could not load configuration: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    pub INTEGRATIONS_AZURE_DI_ENDPOINT: String,
    pub INTEGRATIONS_AZURE_DI_KEY: String,

    pub ADDRESS_URL: String,
    pub DB_URL: String,
    pub IS_AUTOLOGIN: bool,
    pub IS_BYPASS_GUIDE: bool,
    pub IS_DEMO: bool,
    pub IS_NO_SIGNUPS: bool,
    pub IS_PRODUCTION: bool,
    pub IS_FFMPEG_INSTALLED: bool,

    pub PATHS: Paths,
}

#[allow(non_snake_case)]
pub struct Paths {
    BACKUP: PathBuf,
    DB: PathBuf,
    IMAGES: PathBuf,
    LOGS: PathBuf,
    THUMBNAILS: PathBuf,
    VIDEOS: PathBuf,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<Self> {
        Ok(Self {
            INTEGRATIONS_AZURE_DI_ENDPOINT: get_env("SERVICE_INTEGRATIONS_AZURE_DI_ENDPOINT")?,
            INTEGRATIONS_AZURE_DI_KEY: get_env("SERVICE_INTEGRATIONS_AZURE_DI_KEY")?,

            ADDRESS_URL: get_env("SERVICE_ADDRESS_URL")?,
            DB_URL: get_env("DATABASE_URL")?,
            IS_AUTOLOGIN: get_env("SERVICE_AUTOLOGIN")?.to_lowercase() == "true",
            IS_BYPASS_GUIDE: get_env("SERVICE_BYPASS_GUIDE")?.to_lowercase() == "true",
            IS_DEMO: get_env("SERVICE_DEMO")?.to_lowercase() == "true",
            IS_NO_SIGNUPS: get_env("SERVICE_NO_SIGNUPS")?.to_lowercase() == "true",
            IS_PRODUCTION: get_env("SERVICE_PRODUCTION")?.to_lowercase() == "true",
            IS_FFMPEG_INSTALLED: check_ffmpeg(),

            PATHS: Paths::new()?,
        })
    }
}

fn check_ffmpeg() -> bool {
    match Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            child.wait().unwrap();
            true
        }
        Err(_) => {
            print!("\x1b[31mX\x1b[0m Could not find ffmpeg");
            #[cfg(target_os = "macos")]
            {
                println!("\tPlease execute: brew install ffmpeg");
            }

            #[cfg(target_os = "linux")]
            {
                println!("\tPlease consult your package manager to install it.");
            }

            #[cfg(target_os = "windows")]
            {
                println!("Attempting to install using winget");
                if let Ok(mut child) = Command::new("winget")
                    .args(["install", "FFmpeg (Essentials Build)"])
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    child.wait().unwrap();
                    println!("\x1b[32mOK\x1b[0m FFmpeg installed");
                    println!(
                        "Please reload your command prompt to refresh the environment variables."
                    );
                    std::process::exit(1);
                } else {
                    print!("\x1b[31mX\x1b[0m Failed to install using winget. Please install manually: https://www.gyan.dev/ffmpeg/builds");
                }
            }

            false
        }
    }
}

impl Paths {
    fn new() -> lib_utils::envs::Result<Self> {
        let mut root = dirs::config_dir().expect("failed to get config directory path");
        root.push("Recipya2");

        let backup = root.join("Backup");
        let db = root.join("Database");
        let images = root.join("Images");
        let logs = root.join("Logs");
        let thumbnails = images.join("Thumbnails");
        let videos = root.join("Videos");

        fs::create_dir_all(&backup).unwrap();
        fs::create_dir_all(&db).unwrap();
        fs::create_dir_all(&logs).unwrap();
        fs::create_dir_all(&thumbnails).unwrap();
        fs::create_dir_all(&videos).unwrap();

        let stars = "*".repeat(backup.to_str().unwrap().len() + 20);
        info!("{stars}");
        info!("File locations:");
        info!("\tBackups:  {}", backup.display());
        info!("\tDatabase: {}", db.display());
        info!("\tImages:   {}", images.display());
        info!("\tLogs:     {}", logs.display());
        info!("\tVideos:   {}", videos.display());
        info!("{stars}");

        Ok(Paths {
            BACKUP: backup,
            DB: db,
            IMAGES: images,
            LOGS: logs,
            THUMBNAILS: thumbnails,
            VIDEOS: videos,
        })
    }
}
