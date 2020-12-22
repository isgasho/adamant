use super::{Io, StrWrite};
use crate::x86::portio;
use core::fmt::{ Write };
use spin::Mutex;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum COMPort {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
}

pub struct SerialHandle {
    port: COMPort,
}

impl SerialHandle {
    pub const fn new(port: COMPort) -> Self {
        Self { port }
    }
}

impl StrWrite for SerialHandle {}

impl Io for SerialHandle {
    fn write_byte(&self, value: u8) {
        portio::inb(self.port as u16, value);
    }

    fn get_byte(&self) -> u8 {
        todo!();
    }
}

impl Write for SerialHandle {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    static ref HANDLE: Mutex<SerialHandle> = Mutex::new(SerialHandle::new(COMPort::COM1));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{} at {}: {}\n",file!(), column!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::serial::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    HANDLE.lock().write_fmt(args).unwrap();
}
