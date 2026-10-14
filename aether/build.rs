use std::process::Command;
fn main() {
    println!("cargo:rerun-if-env-changed=AETHER_BUILD_REVISION");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=../.git/HEAD");
    let git = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    };
    if let Some(branch) = git(&["symbolic-ref", "HEAD"]) {
        println!("cargo:rerun-if-changed=../.git/{branch}");
    }
    let revision = std::env::var("AETHER_BUILD_REVISION")
        .ok()
        .or_else(|| {
            let revision = git(&["rev-parse", "HEAD"])?;
            let clean = Command::new("git")
                .args(["diff", "--quiet", "HEAD"])
                .status()
                .ok()?
                .success();
            Some(if clean {
                revision
            } else {
                format!("{revision}-dirty")
            })
        })
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=AETHER_BUILD_REVISION={revision}");
}
