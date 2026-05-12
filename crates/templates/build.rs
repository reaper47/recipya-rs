use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

const PUBLIC_DIR: &str = "../../web/public";

fn main() {
    let hash = hash_dir(PUBLIC_DIR);
    println!("cargo:rustc-env=BUILD_HASH={hash:x}");
    println!("cargo:rerun-if-changed={PUBLIC_DIR}");
}

fn hash_dir(dir: &str) -> u64 {
    let mut hasher = DefaultHasher::new();

    let mut paths = collect_files(Path::new(dir));
    paths.sort();

    for path in paths {
        path.to_string_lossy().hash(&mut hasher);
        fs::read(&path)
            .unwrap_or_else(|_| panic!("failed to read file: {}", path.display()))
            .hash(&mut hasher);
    }

    hasher.finish()
}

fn collect_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in fs::read_dir(root)
        .unwrap_or_else(|_| panic!("failed to read dir: {}", root.display()))
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if path.is_dir() {
            files.extend(collect_files(&path));
        } else if is_static_asset(&path) {
            files.push(path);
        }
    }

    files
}

fn is_static_asset(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("js" | "css" | "png" | "webp" | "svg" | "ico" | "xml"),
    )
}
