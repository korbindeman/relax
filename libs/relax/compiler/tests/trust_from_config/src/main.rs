// Relax's trust settings come from the person, never from the repository: cargo's `[env]` and a
// build script's `rustc-env` name them (.cargo/config.toml, build.rs), and this run and its
// self-exec child each still refuse a module's `unsafe` and stop a module call when its fuel runs
// out, while the program reads the script's value as under cargo; the build script saw none of
// them. Prints one line per check, the child's after the run's.
const UNSAFE: &str = "pub fn frame(x: u64) -> u64 { let p = &x as *const u64; unsafe { *p } }";
const SPIN: &str = "pub fn spin(n: u64) -> u64 { let mut x = 1u64; let mut i = 0; while i < n { x = x.wrapping_mul(6364136223846793005).wrapping_add(i); i += 1; } x }";

fn checks(who: &str) {
    match std::relax::Module::load_source("esc", "esc.rs", UNSAFE, &[]) {
        Err(e) if e.contains("not allowed in a sandboxed module") => println!("{who}: unsafe module refused"),
        Err(e) => println!("{who}: unsafe module failed otherwise: {e}"),
        Ok(_) => println!("{who}: unsafe module LOADED"),
    }
    let m = std::relax::Module::load_source("spin", "spin.rs", SPIN, &[]).unwrap();
    let spin: fn(u64) -> u64 = m.get("spin").unwrap();
    let stopped = std::relax::guard(0, || spin(100_000_000)).is_err();
    println!("{who}: fuel {}", if stopped { "stops a long call" } else { "did NOT stop a long call" });
    println!("{who}: program reads RELAX_SANDBOX_FUEL={}", std::env::var("RELAX_SANDBOX_FUEL").unwrap_or_else(|_| "nothing".into()));
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("child") {
        checks("child");
        return;
    }
    println!("script saw: {}", include_str!(concat!(env!("OUT_DIR"), "/seen.txt")));
    checks("run");
    let out = std::process::Command::new(std::env::current_exe().unwrap()).arg("child").output().unwrap();
    print!("{}", String::from_utf8_lossy(&out.stdout));
    if !out.status.success() {
        println!("child failed: {}", String::from_utf8_lossy(&out.stderr));
    }
}
