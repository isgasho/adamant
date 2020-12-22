pub mod serial;

pub trait Io {
    fn write_byte(&self, value: u8);

    fn get_byte(&self) -> u8;
}

pub trait StrWrite: Io {
    fn write_string(&self, buffer: &str) {
        for b in buffer.as_bytes() {
            self.write_byte(*b);
        }
    }
}
