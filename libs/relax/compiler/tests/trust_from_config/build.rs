// A path package's build script: what it sees of Relax's trust settings, and `rustc-env` lines
// that name two of them (they set variables for the program, never for Relax).
fn main() {
    let names = ["RELAX_MODULES_TRUSTED", "RELAX_SANDBOX_NOCHECK", "RELAX_ALLOW_C", "RELAX_SANDBOX_FUEL"];
    let seen: Vec<String> = names.iter().filter_map(|n| std::env::var(n).ok().map(|v| format!("{n}={v}"))).collect();
    let out = std::env::var("OUT_DIR").unwrap();
    std::fs::write(format!("{out}/seen.txt"), if seen.is_empty() { "none".to_string() } else { seen.join(" ") }).unwrap();
    println!("cargo:rustc-env=RELAX_SANDBOX_NOCHECK=1");
    println!("cargo:rustc-env=RELAX_SANDBOX_FUEL=0");
}
