#![no_std]

extern crate ros_lib;
#[macro_use] extern crate bitflags;

use volatile::Volatile;
use ros_lib::io::{ Writer, TextWriter };

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

bitflags! {
    pub struct PrintFlags: u8 {
        const ERROR = 0b00000001;
        const NO_WRITE = 0b00000010;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaWriter {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl VgaWriter {
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn new() -> VgaWriter {
        VgaWriter {
            column_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
        }
    }
}

impl Writer for VgaWriter {
    fn write_byte(&mut self, byte: u8) {
        let flags = PrintFlags::empty();
        if flags.contains(PrintFlags::NO_WRITE) { return; }

        if byte == b'\n' {
            self.new_line();
            return;
        }


        if self.column_position >= BUFFER_WIDTH {
            self.new_line();
        }

        self.buffer.chars[BUFFER_HEIGHT - 1][self.column_position].write(ScreenChar {
            ascii_character: byte,
            color_code: if flags.contains(PrintFlags::ERROR) { ColorCode(0x4F) } else { self.color_code }
        });

        self.column_position += 1;
    }
}

impl TextWriter for VgaWriter {}

impl core::fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}