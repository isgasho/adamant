#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[macro_use]
extern crate lazy_static;

pub static STACK: [u8; 4096] = [0; 4096];

pub mod io;
pub mod x86;
pub mod header;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main(_stivale_struct_addr: usize) -> ! {
    println!("Salut ca gaze");
    
    x86::x86_64::gdt::load_gdt();

    loop {}
}

// TODO: Print panic infos into the VGA buffer
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
