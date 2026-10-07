//! Records where and when the binary was built, for `--version`.

use std::process::Command;

fn main() {
    let run = |cmd: &str, args: &[&str]| {
        Command::new(cmd)
            .args(args)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    };
    println!("cargo:rustc-env=BUILD_HOST={}", run("hostname", &[]));
    println!(
        "cargo:rustc-env=BUILD_COMMIT={}",
        run("git", &["rev-parse", "--short", "HEAD"])
    );
    println!(
        "cargo:rustc-env=BUILD_TIME={}",
        run("date", &["+%Y-%m-%dT%H:%M:%S%z"])
    );
    println!("cargo:rerun-if-changed=../../.git/HEAD");
}
