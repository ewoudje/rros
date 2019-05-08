#![no_std]

extern crate x86_64;

pub unsafe fn exit_qemu() {
    debug!("Closing VM");

    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}