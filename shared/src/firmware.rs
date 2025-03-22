#![no_std]

// Constants for memory addresses
pub mod addresses {
    // Boot section
    pub const BOOT_ADDR: u32 = 0x08000000;
    
    // Loader section
    pub const LOADER_ADDR: u32 = 0x08004000;
    
    // Updater section
    pub const UPDATER_ADDR: u32 = 0x08008000;
    
    // Application section
    pub const APP_ADDR: u32 = 0x08020000;
    
    // Slots from your C code
    pub const SLOT_1_APP_LOADER_ADDR: u32 = 0x08004000;
    pub const SLOT_2_APP_ADDR: u32 = 0x08020000;
    pub const SLOT_2_VER_ADDR: u32 = 0x08020000;
    pub const BACKUP_ADDR: u32 = 0x080A0000;
    pub const PATCH_ADDR: u32 = 0x080C0000;
}