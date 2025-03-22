use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_name = env::var("CARGO_PKG_NAME").unwrap();
    
    let memory_x = fs::read_to_string("memory.x").unwrap();
    fs::write(format!("{}/memory.x", out_dir), memory_x).unwrap();
    
    println!("cargo:rustc-link-search={}", out_dir);
    
    // Post-build commands to create binary
    let target_dir = Path::new(&out_dir).ancestors().nth(4).unwrap();
    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    let elf_path = target_dir.join(format!("{}/{}", profile, target_name));
    
    println!("cargo:warning=Building binary from {}", elf_path.display());
    
    // Run objcopy after build to create the binary
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-arg=-Wl,--print-memory-usage");
    }
}