#![no_std]
#![no_main]
#![feature(llvm_asm)]

pub mod io;
pub mod x86;

use core::panic::PanicInfo;
use io::{serial::*, *};
use stivale::{HeaderFramebufferTag, StivaleHeader};

static STACK: [u8; 4096] = [0; 4096];
static FRAMEBUFFER_TAG: HeaderFramebufferTag = HeaderFramebufferTag::new().bpp(24);

#[link_section = ".stivale2hdr"]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new(STACK[0] as *const u8)
    .tags((&FRAMEBUFFER_TAG as *const HeaderFramebufferTag).cast());

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
