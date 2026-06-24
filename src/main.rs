#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::write_volatile;
use bootloader_api::{entry_point, BootInfo};

const VGA_TEXT_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_LIGHT_GREEN_ON_BLACK: u8 = 0x0a;

fn halt() {
    unsafe {
        asm!("hlt");
    }
}

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    write_screen_byte(0, b'M', VGA_LIGHT_GREEN_ON_BLACK);

    loop {
        halt();
    }
}

fn write_screen_byte(cell: usize, character: u8, color: u8) {
    let byte_offset = cell * 2;

    unsafe {
        write_volatile(VGA_TEXT_BUFFER.add(byte_offset), character);
        write_volatile(VGA_TEXT_BUFFER.add(byte_offset + 1), color);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        halt();
    }
}
