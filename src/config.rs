use crate::{Error, Result};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::{env, fs};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| panic!("Could not load configuration: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub EMAIL_FROM: String,
    pub EMAIL_SENDGRID_API_KEY: String,

    pub INTEGRATIONS_AZURE_DI_ENDPOINT: String,
    pub INTEGRATIONS_AZURE_DI_KEY: String,

    pub ADDRESS: String,
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
    pub DOCS: PathBuf,
    IMAGES: PathBuf,
    LOGS: PathBuf,
    pub STATIC: PathBuf,
    THUMBNAILS: PathBuf,
    VIDEOS: PathBuf,
}

impl Config {
    fn load_from_env() -> Result<Self> {
        Ok(Self {
            EMAIL_FROM: get_env("SERVICE_EMAIL_FROM")?,
            EMAIL_SENDGRID_API_KEY: get_env("SERVICE_EMAIL_SENDGRID_API_KEY")?,

            INTEGRATIONS_AZURE_DI_ENDPOINT: get_env("SERVICE_INTEGRATIONS_AZURE_DI_ENDPOINT")?,
            INTEGRATIONS_AZURE_DI_KEY: get_env("SERVICE_INTEGRATIONS_AZURE_DI_KEY")?,

            ADDRESS: get_env("SERVICE_ADDRESS")?,
            DB_URL: get_env("SERVICE_DB_URL")?,
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
                    process::exit(1);
                } else {
                    print!("\x1b[31mX\x1b[0m Failed to install using winget. Please install manually: https://www.gyan.dev/ffmpeg/builds");
                }
            }

            false
        }
    }
}

impl Paths {
    fn new() -> Result<Self> {
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

        println!("\nFile locations:");
        println!("\t- Backups:  {}", backup.display());
        println!("\t- Database: {}", db.display());
        println!("\t- Images:   {}", images.display());
        println!("\t- Logs:     {}", logs.display());
        println!("\t- Videos:   {}", videos.display());

        Ok(Paths {
            BACKUP: backup,
            DB: db,
            DOCS: PathBuf::from(get_env("SERVICE_DOCS_FOLDER")?),
            IMAGES: images,
            LOGS: logs,
            STATIC: PathBuf::from(get_env("SERVICE_STATIC_FOLDER")?),
            THUMBNAILS: thumbnails,
            VIDEOS: videos,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name)
        .map(|v| v.trim_matches('"').to_string())
        .map_err(|_| Error::ConfigMissingEnv(name))
}
