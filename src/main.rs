#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use bootloader_api::{entry_point, BootInfo};

fn halt() {
    unsafe {
        asm!("hlt");
    }
}

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    loop {
        halt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        halt();
    }
}
