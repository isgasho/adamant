#![no_std]
#![no_main]
#![feature(llvm_asm)]

pub mod io;
pub mod x86;
pub mod header;

use core::panic::PanicInfo;
use io::{serial::*, *};

#[no_mangle]
pub extern "C" fn kernel_main(_stivale_struct_addr: usize) -> ! {
    let mut serial = SerialHandle::new(COMPort::COM1);
    serial.write_str("Salut ca gaze");

    loop {}
}

// TODO: Print panic infos into the VGA buffer
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
