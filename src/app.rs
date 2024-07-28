use crate::services::email::Sendgrid;
use std::{io, io::Write, sync::Arc};

refinery::embed_migrations!("migrations");

pub struct App {
    pub email: Option<Arc<Sendgrid>>,
}

impl App {
    pub async fn new() -> Self {
        // Setup config file
        /*let is_running_in_docker = Path::new("/.dockerenv").exists();
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

                config.server.database_url = prompt_user(
                    "What is your PostgreSQL URL? [postgres://user:password@localhost/recipya]",
                    "",
                );

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
        println!("\x1b[32mOK\x1b[0m Configuration file");*/

        // Connect to database
        //let repository = PsqlRepository::new().await.unwrap();
        //println!("\x1b[32mOK\x1b[0m Database connection");

        // Setup FDC database
        // TODO: Figure out what to do with FDC database
        /*let fdc_db_path = paths.db.join("fdc.db");
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
        }*/

        /*let email = match Sendgrid::new() {
            None => None,
            Some(sg) => Some(Arc::new(sg)),
        };*/

        Self {
            email: None,
            //repository: Arc::new(repository),
        }
    }

    pub fn new_test() -> Self {
        App {
            email: None,
            //repository: Arc::new(MockRepository::default()),
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
