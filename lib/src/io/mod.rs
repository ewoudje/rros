use core::fmt::{Write, Error};

pub trait Writer {
    fn write_byte(&mut self, byte: u8);

    fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes { self.write_byte(byte) }
    }
}

pub trait TextWriter: Writer {
    fn print(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }

    fn println(&mut self, string: &str) {
        self.print(string);
        self.write_byte(b'\n');
    }
}

impl Write for TextWriter {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.print(s);
        return Ok(());
    }
}