#![no_std]
#![feature(const_panic)]
#![feature(abi_x86_interrupt)]

extern crate spin;
extern crate x86_64;
extern crate ros_alloc;

use core::borrow::{BorrowMut, Borrow};
use ros_alloc::{Alloc, MutexAlloc};

#[global_allocator]
static mut ALLOC: MutexAlloc = unsafe { MutexAlloc::empty() };

#[macro_use] pub mod macros;
pub mod io;
pub mod text_output;
pub mod interrupts;
pub mod alloc;

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| unsafe { (**text_output::TEXT_OUT.lock()).write_fmt(args).unwrap() });
}

#[doc(hidden)]
pub fn _debug(args: core::fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| unsafe { (**text_output::TEXT_DEBUG.lock()).write_fmt(args).unwrap() });
}
