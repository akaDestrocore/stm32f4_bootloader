#![no_std]

// Константы для адресов памяти
pub mod addresses {
    // Секция загрузчика
    pub const BOOT_ADDR: u32 = 0x08000000;
    
    // Секция загрузчика приложений
    pub const LOADER_ADDR: u32 = 0x08004000;
    
    // Секция программы обновления
    pub const UPDATER_ADDR: u32 = 0x08008000;
    
    // Секция приложения
    pub const APP_ADDR: u32 = 0x08020000;
    
    // Слоты из C кода
    pub const SLOT_1_APP_LOADER_ADDR: u32 = 0x08004000;
    pub const SLOT_2_APP_ADDR: u32 = 0x08020000;
    pub const SLOT_2_VER_ADDR: u32 = 0x08020000;
    pub const BACKUP_ADDR: u32 = 0x080A0000;
    pub const PATCH_ADDR: u32 = 0x080C0000;
}