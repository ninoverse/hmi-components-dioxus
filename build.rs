use std::fs;
use std::path::Path;
use std::process::Command;

const PACKAGE: &str = "@ninoverse/hmi-components";
const VERSION: &str = "3.1.2";
const TARBALL: &str = "ninoverse-hmi-components-3.1.2.tgz";

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

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    for (_, out) in FILES {
        println!("cargo:rerun-if-changed=assets/vendor/{out}");
    }

    if FILES
        .iter()
        .all(|(_, out)| Path::new("assets/vendor").join(out).exists())
    {
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

    for (src, out) in FILES {
        fs::copy(tmp.join(src), Path::new("assets/vendor").join(out))
            .unwrap_or_else(|e| panic!("failed to copy {src}: {e}"));
    }

    fs::remove_dir_all(&tmp).ok();
}
