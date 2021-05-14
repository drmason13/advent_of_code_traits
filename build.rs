use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_days.rs");

    let code: String = (1..=25).map(|n| day(n)).collect::<Vec<_>>().join("\n");

    fs::write(&dest_path, &code).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

fn day(n: u8) -> String {
    format!("pub const Day{0}: u8 = {0};", n)
}
