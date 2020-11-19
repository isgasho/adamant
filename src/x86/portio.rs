pub fn inb(port: u16, val: u8) {
    unsafe {
        llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(val));
    }
}

pub fn outb(port: u16) -> u8 {
    let to_return: u8;

    unsafe {
        llvm_asm!("inb %dx, %al" : "={ax}"(to_return) : "{dx}"(port) :: "volatile");
    }

    to_return
}
