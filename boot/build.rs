use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Выводим инструкции для cargo перестроить при изменении этих файлов
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=device.x");
    println!("cargo:rerun-if-changed=build.rs");
    
    // Устанавливаем переменную среды для cortex-m-rt
    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());
    
    // Копируем memory.x в каталог OUT_DIR
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy("memory.x", out.join("memory.x")).unwrap();
    fs::copy("device.x", out.join("device.x")).unwrap();
    
    // Только для отладки - сообщаем местоположение memory.x
    println!("cargo:warning=memory.x location: {}", out.join("memory.x").display());
}