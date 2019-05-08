#![macro_use]

use spin::Mutex;
use crate::io::{TextWriter, Writer};
use ros_mem::global::make_global;
use core::ptr::null_mut;

struct DummyWriter;

impl Writer for DummyWriter {
    fn write_byte(&mut self, byte: u8) {
        unimplemented!()
    }
}

impl TextWriter for DummyWriter {}


pub static mut TEXT_OUT: Mutex<*mut TextWriter> = Mutex::new(null_mut::<DummyWriter>());
pub static mut TEXT_DEBUG: Mutex<*mut TextWriter> = Mutex::new(null_mut::<DummyWriter>());

pub unsafe fn init_text_drivers<TO: 'static + TextWriter, TD: 'static + TextWriter>(to: TO, td: TD) {
    TEXT_OUT = Mutex::new(make_global::<TO>(to));
    TEXT_DEBUG = Mutex::new(make_global::<TD>(td));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::_debug(format_args!("{}\n", format_args!($($arg)*))));
}