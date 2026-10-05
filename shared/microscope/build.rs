//! Build provenance for the footer (as the X_eTaL live demo shows it):
//! the build host, this repo's short commit, the build time, and the
//! pinned X_eTaL commit (XETAL_COMMIT).

use std::process::Command;

fn run(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

fn pinned() -> String {
    let text = std::fs::read_to_string("../../XETAL_COMMIT").unwrap_or_default();
    match text.trim() {
        "" => "unknown".into(),
        c => c.chars().take(7).collect(),
    }
}

fn main() {
    println!("cargo:rustc-env=BUILD_SHA={}", run("git", &["rev-parse", "--short", "HEAD"]));
    println!("cargo:rustc-env=BUILD_HOST={}", run("hostname", &["-s"]));
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", run("date", &["-u", "+%Y%m%dT%H%M%S"]));
    println!("cargo:rustc-env=XETAL_SHA={}", pinned());
    println!("cargo:rerun-if-changed=../../XETAL_COMMIT");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
}
