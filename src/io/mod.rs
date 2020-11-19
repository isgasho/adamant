pub mod serial;

pub trait Io {

    fn write_byte(&mut self, value: u8);

    fn get_byte(&mut self) -> u8;
}

pub trait StrWrite: Io {
    fn write_str(&mut self, buffer: &str) {
        for b in buffer.as_bytes() {
            self.write_byte(*b);
        }
    }
}
