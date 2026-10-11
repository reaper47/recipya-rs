use std::{
    env, fs,
    path::{Path, PathBuf},
};

use fluent_static_codegen::MessageBundleBuilder;

const DEFAULT_LANGUAGE: &str = "en-CA";
const RESOURCE_FILE_NAME: &str = "l10n.ftl";

pub fn main() {
    println!("cargo::rerun-if-changed=locales/");

    let base_dir = resources_base_dir();

    let mut builder = MessageBundleBuilder::new("Messages");

    builder
        .set_default_language(DEFAULT_LANGUAGE)
        .expect("default language should be valid identifier")
        .set_resources_dir(base_dir.clone());

    for tag in locale_tags(&base_dir) {
        builder
            .add_resource(&tag, format!("{tag}/{RESOURCE_FILE_NAME}"))
            .unwrap_or_else(|err| panic!("locales/{tag}/{RESOURCE_FILE_NAME} is invalid: {err:?}"));
    }

    builder
        .build()
        .unwrap()
        .write_to_file(output_dir().join("messages.rs"))
        .expect("Output directory should exist and be writeable to save generated code");
}

fn resources_base_dir() -> PathBuf {
    PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("'CARGO_MANIFEST_DIR' not set"))
        .join("locales")
}

fn output_dir() -> PathBuf {
    let out =
        PathBuf::from(env::var_os("OUT_DIR").expect("'OUT_DIR' not set")).join("generated/fluent");
    if !out.exists() {
        fs::create_dir_all(&out).unwrap();
    }
    out
}

fn locale_tags(base: &Path) -> Vec<String> {
    // When building for production, all locales will be included. It will take a long time to compile the crate.
    // When building in debug mode, only the en-CA locale will be included. This is to minimize compile times.
    let is_debug_only = env::var("PROFILE").is_ok_and(|p| p == "debug");

    let mut tags = fs::read_dir(base)
        .expect("locales directory should exist")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().join(RESOURCE_FILE_NAME).is_file())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|tag| !is_debug_only || tag == DEFAULT_LANGUAGE)
        .collect::<Vec<_>>();

    tags.sort();
    tags
}
