use std::path::Path;

use walkdir::{DirEntry, WalkDir};

const LOCALES_DIR: &str = "locales";

pub fn run_i18n(subcommands: &[String], options: &[String]) {
    dbg!((&subcommands, &options));
    for entry in WalkDir::new(LOCALES_DIR)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(is_ftl)
    {
        println!("{}", entry.path().display());
    }
}

fn is_ftl(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
        && entry.file_name().to_str().is_some_and(|s| {
            Path::new(s)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("ftl"))
        })
}
