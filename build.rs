use std::process::Command;

fn get_sha() -> String {
    let o = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("failed to execute process");
    String::from_utf8(o.stdout)
        .expect("to have a hash")
        .trim()
        .to_string()
}

fn main() {
    println!("cargo:rustc-env=RUSTAROO_SHA={}", get_sha());
}
