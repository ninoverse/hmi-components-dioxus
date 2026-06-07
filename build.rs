use std::fs;
use std::path::Path;
use std::process::Command;

const PACKAGE: &str = "@ninoverse/hmi-components";
const VERSION: &str = "3.1.2";
const TARBALL: &str = "ninoverse-hmi-components-3.1.2.tgz";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/vendor/hmi-components.iife.js");
    println!("cargo:rerun-if-changed=assets/vendor/hmi-components.css");

    let out_js = Path::new("assets/vendor/hmi-components.iife.js");
    let out_css = Path::new("assets/vendor/hmi-components.css");

    if out_js.exists() && out_css.exists() {
        return;
    }

    fs::create_dir_all("assets/vendor").expect("failed to create assets/vendor");

    let tmp = std::env::temp_dir().join("hmi-components-fetch");
    fs::create_dir_all(&tmp).expect("failed to create temp dir");

    let status = Command::new("npm")
        .args(["pack", &format!("{PACKAGE}@{VERSION}")])
        .current_dir(&tmp)
        .status()
        .expect("npm not found — install Node.js to fetch hmi-components");
    assert!(status.success(), "npm pack failed");

    let status = Command::new("tar")
        .args([
            "xzf",
            tmp.join(TARBALL).to_str().unwrap(),
            "-C",
            tmp.to_str().unwrap(),
            "--strip-components=2",
            "package/dist/hmi-components.iife.js",
            "package/dist/hmi-components.css",
        ])
        .status()
        .expect("tar not found");
    assert!(status.success(), "tar extraction failed");

    fs::copy(tmp.join("hmi-components.iife.js"), out_js).expect("failed to copy iife.js");
    fs::copy(tmp.join("hmi-components.css"), out_css).expect("failed to copy css");

    fs::remove_dir_all(&tmp).ok();
}
