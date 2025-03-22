// build_all.rs - Custom build script to build and merge all components
// Usage: cargo run --release --bin build_all

use std::process::Command;
use std::{fs, env, path::Path};

fn main() {
    println!("Building STM32F4 firmware components...");
    
    // Build all components
    build_component("boot");
    build_component("loader");
    build_component("updater");
    build_component("application");
    
    // Convert ELF to BIN for each component
    let target_dir = "target/thumbv7em-none-eabihf/release";
    convert_to_bin(target_dir, "boot");
    convert_to_bin(target_dir, "loader");
    convert_to_bin(target_dir, "updater");
    convert_to_bin(target_dir, "application");
    
    // Patch headers using existing Python script
    patch_headers();
    
    // Merge binaries using existing Python script
    merge_binaries();
    
    println!("Build complete! Final firmware is merged_firmware.bin");
}

fn build_component(name: &str) {
    println!("Building {}...", name);
    let status = Command::new("cargo")
        .args(["build", "--release", "--bin", name])
        .status()
        .expect("Failed to execute cargo build");
        
    if !status.success() {
        panic!("Failed to build {}", name);
    }
}

fn convert_to_bin(target_dir: &str, name: &str) {
    println!("Converting {} to binary...", name);
    
    let elf_path = format!("{}/{}", target_dir, name);
    let bin_path = format!("{}/{}.bin", target_dir, name);
    
    let objcopy = env::var("OBJCOPY").unwrap_or_else(|_| "arm-none-eabi-objcopy".to_string());
    
    let status = Command::new(&objcopy)
        .args(["-O", "binary", &elf_path, &bin_path])
        .status()
        .expect("Failed to execute objcopy");
        
    if !status.success() {
        panic!("Failed to convert {} to binary", name);
    }
}

fn patch_headers() {
    println!("Patching headers...");
    
    let target_dir = "target/thumbv7em-none-eabihf/release";
    
    // Call the existing Python script for each binary
    let status = Command::new("python")
        .args([
            "scripts/patch_image_header.py", 
            &format!("{}/boot.bin", target_dir),
            "--skip-header"
        ])
        .status()
        .expect("Failed to patch boot header");
        
    if !status.success() {
        panic!("Failed to patch boot header");
    }
    
    // Patch loader header
    let status = Command::new("python")
        .args([
            "scripts/patch_image_header.py", 
            &format!("{}/loader.bin", target_dir)
        ])
        .status()
        .expect("Failed to patch loader header");
        
    if !status.success() {
        panic!("Failed to patch loader header");
    }
    
    // Patch updater header
    let status = Command::new("python")
        .args([
            "scripts/patch_image_header.py", 
            &format!("{}/updater.bin", target_dir)
        ])
        .status()
        .expect("Failed to patch updater header");
        
    if !status.success() {
        panic!("Failed to patch updater header");
    }
    
    // Patch application header
    let status = Command::new("python")
        .args([
            "scripts/patch_image_header.py", 
            &format!("{}/application.bin", target_dir)
        ])
        .status()
        .expect("Failed to patch application header");
        
    if !status.success() {
        panic!("Failed to patch application header");
    }
}

fn merge_binaries() {
    println!("Merging binaries...");
    
    let target_dir = "target/thumbv7em-none-eabihf/release";
    
    // Call the existing Python script to merge binaries
    let status = Command::new("python")
        .args([
            "scripts/merge_images.py", 
            &format!("{}/boot.bin", target_dir),
            &format!("{}/loader.bin", target_dir),
            &format!("{}/updater.bin", target_dir),
            &format!("{}/application.bin", target_dir)
        ])
        .status()
        .expect("Failed to merge binaries");
        
    if !status.success() {
        panic!("Failed to merge binaries");
    }
    
    // Copy the merged firmware to the project root
    fs::copy("merged_firmware.bin", "final_firmware.bin")
        .expect("Failed to copy merged firmware");
}