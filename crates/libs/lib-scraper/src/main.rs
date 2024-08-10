fn main() {
    #[cfg(feature = "refresh_html")]
    refresh_html();
}

#[cfg(feature = "refresh_html")]
fn refresh_html() {
    let client = reqwest::blocking::Client::new();

    let map = lib_scraper::websites();
    let i = 1;
    let total = map.len();

    for (website, url) in map.into_iter() {
        print!("\rProcessing {}/{}: {}", i, total, website);

        match client.get(url).send() {
            Ok(res) => {
                std::fs::File::File::create(std::path::PathBuf::PathBuf::from(format!(
                    "./crates/libs/lib-scraper/tests/data/{}.html",
                    website
                )))
                .unwrap_or_else(|_| {
                    std::fs::File::File::create(format!("./tests/data/{}.html", website)).unwrap()
                })
                .write(&res.bytes().unwrap())
                .inspect_err(|err| println!("Could not write {}: {:?}", website, err))
                .unwrap();
            }
            Err(err) => println!("Could not fetch {}: {:?}", website, err),
        };
    }
    println!("\nProcessing complete.");
}
