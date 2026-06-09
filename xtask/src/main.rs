//! Maintainer task: refresh the vendored `@ninoverse/hmi-components` assets.
//!
//! Unlike a build script, this runs only when a maintainer bumps the upstream
//! JS version. It downloads the npm package with `npm pack`, extracts the five
//! files the crate ships, and writes them into the crate's `assets/vendor/`,
//! which is committed so downstream builds (and docs.rs) need no network or npm.
//!
//! Run with: `cargo run -p xtask`

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const PACKAGE: &str = "@ninoverse/hmi-components";
const VERSION: &str = "4.2.0";
const TARBALL: &str = "ninoverse-hmi-components-4.2.0.tgz";

// (path inside the tarball after `package/dist/`, vendored output name).
// The component CSS only consumes design tokens (var(--background), …); the
// theme files below define them, so all four must be vendored together.
const FILES: &[(&str, &str)] = &[
    ("hmi-components.iife.js", "hmi-components.iife.js"),
    ("hmi-components.css", "hmi-components.css"),
    ("themes/constants.css", "hmi-constants.css"),
    ("themes/color/default.css", "hmi-color-default.css"),
    ("themes/structure/default.css", "hmi-structure-default.css"),
];

fn vendor_dir() -> PathBuf {
    // CARGO_MANIFEST_DIR = .../xtask; the vendored assets live one level up in
    // the library crate root (the workspace root), so cwd does not matter.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask manifest dir has a parent")
        .join("assets/vendor")
}

fn main() {
    let out = vendor_dir();
    fs::create_dir_all(&out).expect("failed to create assets/vendor");

    let tmp = std::env::temp_dir().join("hmi-components-fetch");
    fs::create_dir_all(&tmp).expect("failed to create temp dir");

    let status = Command::new("npm")
        .args(["pack", &format!("{PACKAGE}@{VERSION}")])
        .current_dir(&tmp)
        .status()
        .expect("npm not found — install Node.js to fetch hmi-components");
    assert!(status.success(), "npm pack failed");

    let mut tar_args = vec![
        "xzf".to_string(),
        tmp.join(TARBALL).to_str().unwrap().to_string(),
        "-C".to_string(),
        tmp.to_str().unwrap().to_string(),
        "--strip-components=2".to_string(),
    ];
    tar_args.extend(FILES.iter().map(|(src, _)| format!("package/dist/{src}")));

    let status = Command::new("tar")
        .args(&tar_args)
        .status()
        .expect("tar not found");
    assert!(status.success(), "tar extraction failed");

    for (src, name) in FILES {
        fs::copy(tmp.join(src), out.join(name))
            .unwrap_or_else(|e| panic!("failed to copy {src}: {e}"));
        println!("vendored {name}");
    }

    fs::remove_dir_all(&tmp).ok();
    println!("done → {}", out.display());
}
