use super::{Io, StrWrite};
use crate::x86::portio;

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
    pub fn new(port: COMPort) -> Self {
        Self { port }
    }
}

impl Io for SerialHandle {
    fn write_byte(&mut self, value: u8) {
        portio::inb(self.port as u16, value);
    }

    fn get_byte(&mut self) -> u8 {
        todo!();
    }
}

impl StrWrite for SerialHandle {}
