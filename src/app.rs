use std::{env, fs, io, process};
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};
use spinach::{Color, Spinach};
use tokio::net::TcpListener;

pub struct App {
    config: ConfigFile,
    general: General,
    paths: Paths,
}

struct Paths {
    backup: PathBuf,
    db: PathBuf,
    images: PathBuf,
    logs: PathBuf,
    thumbnails: PathBuf,
    videos: PathBuf,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigFile {
    email: ConfigEmail,
    integrations: ConfigIntegrations,
    server: ConfigServer,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigEmail {
    from: String,
    sendgrid_api_key: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigIntegrations {
    azure_di: AzureDI,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AzureDI {
    endpoint: String,
    key: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigServer {
    is_autologin: bool,
    is_bypass_guide: bool,
    is_demo: bool,
    is_no_signups: bool,
    is_production: bool,
    port: u16,
    url: String,
}

struct General {
    is_ffmpeg_installed: bool,
}

impl App {
    pub async fn new() -> Self {
        let paths = Paths::new();

        // Setup config file
        let is_running_in_docker = Path::new("/.dockerenv").exists();
        let config = if is_running_in_docker {
            ConfigFile::new_from_env()
        } else {
            let config_file_path = paths.db.parent().unwrap().join("config.json");
            if config_file_path.exists() {
                let file = File::open(&config_file_path).unwrap();
                ConfigFile::new(file)
            } else {
                println!("\n****************************");
                println!("* Configuration file setup *");
                println!("****************************");

                let mut config = ConfigFile::default();

                let has_send_grid = prompt_user("Do you have a SendGrid account? If not, important emails will not be sent [y/N]", "N");
                if has_send_grid.to_lowercase() == "y" {
                    config.email.from =
                        prompt_user("\tWhat is the email address of your SendGrid account?", "");
                    config.email.sendgrid_api_key =
                        prompt_user("\tWhat is your SendGrid API key?", "");
                }

                let has_ocr = prompt_user("Do you have an Azure AI Document Intelligence account? If not, OCR features will be disabled. [y/N]", "N");
                if has_ocr.to_lowercase() == "y" {
                    config.integrations.azure_di.key =
                        prompt_user("\tWhat is your resource key?", "");
                    config.integrations.azure_di.endpoint =
                        prompt_user("\tWhat is your endpoint?", "");
                }

                let is_autologin = prompt_user("Do you wish to autologin? [y/N]", "N");
                config.server.is_autologin = is_autologin.to_lowercase() == "y";

                let is_no_signups =
                    prompt_user("Do you wish to disable account registrations? [y/N]", "N");
                config.server.is_no_signups = is_no_signups.to_lowercase() == "y";

                let is_prod = prompt_user("Is your application in production? [y/N]", "N");
                config.server.is_production = is_prod.to_lowercase() == "y";

                let url = prompt_user(
                    "What is the app's URL? (default, http://0.0.0.0)",
                    "http://0.0.0.0",
                );
                config.server.url = if cfg!(target_os = "windows") {
                    url.replace("0.0.0.0", "127.0.0.1")
                } else {
                    url
                };

                let listener = TcpListener::bind("127.0.0.1:0")
                    .await
                    .expect("could not bind to address");
                config.server.port = listener.local_addr().unwrap().port();

                let file = File::create_new(config_file_path).unwrap();
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &config).unwrap();

                config
            }
        };
        println!("\x1b[32mOK\x1b[0m Configuration file");

        // Setup FDC database
        let fdc_db_path = paths.db.join("fdc.db");
        if fdc_db_path.exists() {
            println!("\x1b[32mOK\x1b[0m FDC database");
        } else {
            let s = Spinach::new("Fetching the FDC database (62.6 MB)");
            let url = "https://raw.githubusercontent.com/reaper47/recipya/main/deploy/fdc.db.zip";

            match reqwest::get(url).await {
                Err(err) => {
                    let mut msg = String::from("Error downloading FDC database: ");
                    msg.push_str(&err.to_string()[..]);
                    s.stop_with("Error", msg, Color::Red);
                    println!("Application setup will terminate");
                    process::exit(1);
                }
                Ok(res) => {
                    if res.status().is_success() {
                        s.stop_with("OK", "FDC database", Color::Green);

                        let zip_path = Path::new(&paths.db).join("fdc.db.zip");
                        let content = res.bytes().await.unwrap();
                        fs::write(&zip_path, content)
                            .unwrap_or_else(|e| panic!("Failed to write to fdc.db file: {e}"));

                        let file = fs::File::open(&zip_path).unwrap_or_else(|err| {
                            fs::remove_file(&zip_path).unwrap();
                            println!("Failed to open {}: {err}", zip_path.display());
                            process::exit(1);
                        });

                        let mut archive = zip::ZipArchive::new(file).unwrap_or_else(|err| {
                            fs::remove_file(&zip_path).unwrap();
                            println!("Failed to create archive: {err}");
                            process::exit(1);
                        });

                        for i in 0..archive.len() {
                            let mut file = archive.by_index(i).unwrap_or_else(|err| {
                                fs::remove_file(&zip_path).unwrap();
                                println!("Failed to get a file by index in the zip: {err}");
                                process::exit(1);
                            });

                            let out_path = match file.enclosed_name() {
                                Some(path) => Path::new(&paths.db).join(path),
                                None => continue,
                            };

                            if file.is_dir() {
                                fs::create_dir_all(&out_path).unwrap_or_else(|err| {
                                    fs::remove_file(&zip_path).unwrap();
                                    println!(
                                        "Failed to create directory {}: {err}",
                                        &out_path.display()
                                    );
                                    process::exit(1);
                                });
                            } else if let Some(p) = out_path.parent() {
                                if !p.exists() {
                                    fs::create_dir_all(p).unwrap_or_else(|err| {
                                        fs::remove_file(&zip_path).unwrap();
                                        println!(
                                            "Failed to create directory {}: {err}",
                                            &p.display()
                                        );
                                        process::exit(1);
                                    });
                                }
                            }

                            let mut outfile = fs::File::create(&out_path).unwrap_or_else(|err| {
                                fs::remove_file(&zip_path).unwrap();
                                println!("Failed to create file {}: {err}", &out_path.display());
                                process::exit(1);
                            });

                            io::copy(&mut file, &mut outfile).unwrap();

                            #[cfg(unix)]
                            {
                                use std::os::unix::fs::PermissionsExt;

                                if let Some(mode) = file.unix_mode() {
                                    fs::set_permissions(
                                        &out_path,
                                        fs::Permissions::from_mode(mode),
                                    )
                                    .unwrap_or_else(|err| {
                                        fs::remove_file(&zip_path).unwrap();
                                        println!("Failed to set file permissions: {err}");
                                        process::exit(1);
                                    });
                                }
                            }
                        }

                        fs::remove_file(zip_path).unwrap();
                    } else {
                        let mut msg = String::from("Error downloading FDC database: ");
                        msg.push_str(&res.status().to_string()[..]);
                        s.stop_with("Error", msg, Color::Red);
                        println!("Application setup will terminate");
                        process::exit(1);
                    }
                }
            };
        }

        // Verify extra software
        let mut general = General::default();
        if let Ok(mut child) = Command::new("ffmpeg")
            .arg("-version")
            .stdout(Stdio::piped())
            .spawn()
        {
            child.wait().unwrap();
            general.is_ffmpeg_installed = true;
            println!("\x1b[32mOK\x1b[0m FFmpeg installed");
        } else {
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
                let s = Spinach::new("Attempting to install using winget");
                if let Ok(mut child) = Command::new("winget")
                    .args(["install", "FFmpeg (Essentials Build)"])
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    child.wait().unwrap();
                    s.stop_with("OK", "FFmpeg installed", Color::Green);
                    println!("\x1b[32mOK\x1b[0m FFmpeg installed");
                    println!(
                        "Please reload your command prompt to refresh the environment variables."
                    );
                    process::exit(1);
                } else {
                    s.stop_with("X", "Failed to install using winget. Please install manually: https://www.gyan.dev/ffmpeg/builds", Color::Red);
                }
            }
        }

        println!("\nFile locations:");
        println!("\t- Backups:  {}", paths.backup.display());
        println!("\t- Database: {}", paths.db.display());
        println!("\t- Images:   {}", paths.images.display());
        println!("\t- Logs:     {}", paths.logs.display());
        println!("\t- Videos:   {}", paths.videos.display());

        Self {
            paths,
            config,
            general,
        }
    }

    pub fn address(&self, provide_localhost: bool) -> String {
        let is_localhost = self.config.server.url.contains("0.0.0.0")
            || self.config.server.url.contains("localhost")
            || self.config.server.url.contains("127.0.0.1");

        if self.config.server.is_production && !is_localhost {
            if provide_localhost {
                return format!("http://localhost:{}", self.config.server.port);
            }
            return String::from(&self.config.server.url);
        }

        if is_localhost {
            format!("{}:{}", self.config.server.url, self.config.server.port)
        } else {
            String::from(&self.config.server.url)
        }
    }
}

impl ConfigFile {
    fn new<R: Read>(src: R) -> Self {
        serde_json::from_reader(src).unwrap()
    }

    fn new_from_env() -> Self {
        let mut is_env_ok = true;

        let mandatory_env = vec!["RECIPYA_SERVER_PORT"];
        for env in mandatory_env.iter() {
            if env::var(env).is_err() {
                is_env_ok = false;
                println!("Missing required environment variable: {env}");
            }
        }

        if !is_env_ok {
            println!("Application setup will terminate");
            process::exit(1);
        }

        let port = env::var("RECIPYA_SERVER_PORT").unwrap();
        let port: u16 = port.parse().expect("port '{}' must be a number");

        let is_autologin = env::var("RECIPYA_SERVER_AUTOLOGIN").unwrap_or_default() == "true";
        let is_bypass_guide = env::var("RECIPYA_SERVER_BYPASS_GUIDE").unwrap_or_default() == "true";
        let is_demo = env::var("RECIPYA_SERVER_IS_DEMO").unwrap_or_default() == "true";
        let is_no_signups = env::var("RECIPYA_SERVER_NO_SIGNUPS").unwrap_or_default() == "true";
        let is_production = env::var("RECIPYA_SERVER_IS_PROD").unwrap_or_default() == "true";

        let config = Self {
            email: ConfigEmail {
                from: env::var("RECIPYA_EMAIL").unwrap_or_default(),
                sendgrid_api_key: env::var("RECIPYA_EMAIL_SENDGRID").unwrap_or_default(),
            },
            integrations: ConfigIntegrations {
                azure_di: AzureDI {
                    endpoint: env::var("RECIPYA_DI_ENDPOINT").unwrap_or_default(),
                    key: env::var("RECIPYA_DI_KEY").unwrap_or_default(),
                },
            },
            server: ConfigServer {
                is_autologin,
                is_bypass_guide,
                is_demo,
                is_no_signups,
                is_production,
                port,
                url: env::var("RECIPYA_SERVER_URL").unwrap_or(String::from("http://0.0.0.0")),
            },
        };

        println!("\x1b[32mOK\x1b[0m Environment variables");
        config
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            email: ConfigEmail {
                from: String::new(),
                sendgrid_api_key: String::new(),
            },
            integrations: ConfigIntegrations {
                azure_di: AzureDI {
                    endpoint: String::new(),
                    key: String::new(),
                },
            },
            server: ConfigServer {
                is_autologin: false,
                is_bypass_guide: false,
                is_demo: false,
                is_no_signups: false,
                is_production: false,
                port: 8078,
                url: String::from("http://0.0.0.0"),
            },
        }
    }
}

impl Default for General {
    fn default() -> Self {
        Self {
            is_ffmpeg_installed: false,
        }
    }
}

impl Paths {
    fn new() -> Self {
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

        Paths {
            backup,
            db,
            images,
            logs,
            thumbnails,
            videos,
        }
    }
}

fn prompt_user(question: &str, default: &str) -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("\r{} -> ", question);
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        if !input.is_empty() {
            return input.to_string();
        }

        if input.is_empty() && !default.is_empty() {
            return default.to_string();
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn env<'a>() -> HashMap<&'a str, &'a str> {
        HashMap::from([
            ("RECIPYA_DI_ENDPOINT", "https://{res}.azure.com"),
            ("RECIPYA_DI_KEY", "KEY_1"),
            ("RECIPYA_EMAIL", "my@email.com"),
            ("RECIPYA_EMAIL_SENDGRID", "API_KEY"),
            ("RECIPYA_SERVER_IS_DEMO", "false"),
            ("RECIPYA_SERVER_IS_BYPASS_GUIDE", "true"),
            ("RECIPYA_SERVER_IS_PROD", "true"),
            ("RECIPYA_SERVER_PORT", "8078"),
            ("RECIPYA_SERVER_AUTOLOGIN", "true"),
            ("RECIPYA_SERVER_NO_SIGNUPS", "true"),
            ("RECIPYA_SERVER_URL", "localhost"),
        ])
    }

    fn setup_env() {
        for (k, v) in env().iter() {
            env::set_var(k, v);
        }
    }

    fn teardown_env() {
        for (k, _) in env().iter() {
            env::remove_var(k);
        }
    }

    #[test]
    fn config_new_from_env_set_all_fields() {
        setup_env();

        let config = ConfigFile::new_from_env();

        teardown_env();
        assert_eq!(
            config,
            ConfigFile {
                email: ConfigEmail {
                    from: String::from("my@email.com"),
                    sendgrid_api_key: String::from("API_KEY"),
                },
                integrations: ConfigIntegrations {
                    azure_di: AzureDI {
                        endpoint: String::from("https://{res}.azure.com"),
                        key: String::from("KEY_1"),
                    }
                },
                server: ConfigServer {
                    is_autologin: true,
                    is_bypass_guide: true,
                    is_demo: false,
                    is_no_signups: true,
                    is_production: true,
                    port: 8078,
                    url: String::from("localhost"),
                },
            }
        );
    }

    #[test]
    fn config_default() {
        let config = ConfigFile::default();

        assert_eq!(
            config,
            ConfigFile {
                email: ConfigEmail {
                    from: String::new(),
                    sendgrid_api_key: String::new()
                },
                integrations: ConfigIntegrations {
                    azure_di: AzureDI {
                        endpoint: String::new(),
                        key: String::new()
                    }
                },
                server: ConfigServer {
                    is_autologin: false,
                    is_bypass_guide: false,
                    is_demo: false,
                    is_no_signups: false,
                    is_production: false,
                    port: 8078,
                    url: String::from("http://0.0.0.0"),
                },
            }
        )
    }

    #[test]
    fn address_without_port() {
        let mut config = ConfigFile::default();
        config.server.url = String::from("https://localhost");
        let app = App {
            config,
            general: General::default(),
            paths: Paths::new(),
        };

        let got = app.address(true);

        assert_eq!(got, "https://localhost:8078");
    }

    #[test]
    fn address_hosted() {
        let mut config = ConfigFile::default();
        config.server.url = String::from("https://recipya.com");
        let app = App {
            config,
            general: General::default(),
            paths: Paths::new(),
        };

        let got = app.address(false);

        assert_eq!(got, "https://recipya.com");
    }

    #[test]
    fn address_hosted_provide_localhost() {
        let mut config = ConfigFile::default();
        config.server.is_production = true;
        config.server.url = String::from("https://recipya.com");
        let app = App {
            config,
            general: General::default(),
            paths: Paths::new(),
        };

        let got = app.address(true);

        assert_eq!(got, "http://localhost:8078");
    }
}
