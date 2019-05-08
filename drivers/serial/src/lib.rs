#![no_std]

extern crate uart_16550;
extern crate ros_lib;

use ros_lib::io::{ TextWriter, Writer };
use uart_16550::SerialPort;

pub struct Serial(SerialPort);

impl Writer for Serial {
    fn write_byte(&mut self, byte: u8) {
        self.0.send(byte)
    }
}

impl TextWriter for Serial {}

pub unsafe fn new_writer(base : u16) -> Serial {
    let mut port = SerialPort::new(base);
    port.init();
    Serial(port)
}