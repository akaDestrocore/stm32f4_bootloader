#![no_std]

use core::ptr;
use stm32f4 as pac;
use crate::image::ImageHeader;
use crate::flash;

// CRC unit init
pub fn init_crc(p: &pac::Peripherals) {
    // Enable CRC clock
    p.rcc.ahb1enr().modify(|_, w| w.crcen().enabled());
}

pub fn reset_crc(p: &pac::Peripherals) {
    p.crc.cr().write(|w| w.reset().reset());
}
 
pub fn calculate_memory_crc(p: &pac::Peripherals, start_addr: u32, size: u32) -> u32 {

    init_crc(p);    
    reset_crc(p);
    
    // Calculate CRC for 32bit words
    let mut addr: u32 = start_addr;
    let end_addr: u32 = start_addr + size;
    
    while addr + 4 <= end_addr {
        let data: u32 = unsafe { ptr::read_volatile(addr as *const u32) };
        p.crc.dr().write(|w| unsafe { w.dr().bits(data) });
        addr += 4;
    }
    
    // process remaining bytes
    if addr < end_addr {
        let mut remaining_data: u32 = 0;
        let mut shift: i32 = 0;
        
        while addr < end_addr {
            let byte: u8 = unsafe { ptr::read_volatile(addr as *const u8) };
            remaining_data |= (byte as u32) << shift;
            shift += 8;
            addr += 1;
        }
        
        p.crc.dr().write(|w| unsafe { w.dr().bits(remaining_data) });
    }
    
    // read CRC result
    p.crc.dr().read().dr().bits()
}

pub fn verify_firmware_crc(p: &pac::Peripherals, addr: u32, header_size: u32) -> bool {
    let header: ImageHeader = unsafe { ptr::read_volatile(addr as *const ImageHeader) };
    
    // check size
    if header.data_size == 0 || header.data_size > 0x100000 {
        return false;
    }
    
    // only after header
    let data_addr: u32 = addr + header_size;
    
    // calculate firmware crc
    let calculated_crc: u32 = calculate_memory_crc(p, data_addr, header.data_size);
    
    // compare wit CRC from header
    calculated_crc == header.crc
}

pub fn invalidate_firmware(p: &pac::Peripherals, addr: u32) -> bool {
    
    flash::unlock(p);
    
    // just erase the sector that contains header
    let result: u32 = flash::erase_sector(p, addr);
    
    flash::lock(p);
    
    result > 0
}