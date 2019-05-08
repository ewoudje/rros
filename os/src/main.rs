#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]

#[macro_use] extern crate alloc;
extern crate bootloader;
#[macro_use] extern crate ros_lib;
extern crate rosd_serial;
extern crate rosd_vga;
extern crate x86_64;

use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use core::alloc::{AllocErr, Layout};
use core::panic::PanicInfo;

use bootloader::bootinfo::{BootInfo, FrameRange, MemoryRegionType};
use bootloader::entry_point;
use x86_64::PhysAddr;
use x86_64::structures::paging::{PhysFrame, Size4KiB};

use ros_lib::alloc::init_alloc;
use ros_lib::text_output::init_text_drivers;
use ros_mem::{create_alloc_mapping, create_global_mapping};
use ros_mem::alloc::{init_frame_allocator, init_page_allocator};
use ros_mem::global::make_global;
use rosd_vga::VgaWriter;

mod interrupts;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    debug!("{}", info);
    eop!()
}

#[alloc_error_handler]
fn alloc_error(lay: Layout) -> ! {
    debug!("Alloc Error! Size: {}", lay.size());
    eop!()
}
/*
fn prepare_idt(idt: &mut InterruptDescriptorTable) {
    idt.breakpoint.set_handler_fn(breakpoint_handler);
}
*/
entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {

    //INIT INTERRUPTS
    ros_lib::interrupts::init_interrupts(interrupts::interrupts_callback);

    let mut table = unsafe {
        ros_mem::init(boot_info.recursive_page_table_addr as usize)
    };

    //START Region mapping
    let regions = boot_info.memory_map
        .iter()
        .filter(|r| r.region_type == MemoryRegionType::Usable);

    let addr_ranges = regions.map(|r| r.range.start_addr()..r.range.end_addr());

    let frame_addresses = addr_ranges.flat_map(|r| r.into_iter().step_by(4096));

    let frames = frame_addresses.map(|addr| {
        PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(addr))
    });
    //END Region mapping

    let page_allocator;
    let frame_allocator;

    unsafe {
        let mut fallocator = init_frame_allocator(frames);
        let mut pallocator = init_page_allocator(0x0001_0000_0000);
        create_global_mapping(&mut table, &mut fallocator);
        create_alloc_mapping(&mut table, &mut fallocator);
        init_text_drivers(VgaWriter::new(),rosd_serial::new_writer(0x3F8));
        page_allocator = make_global(pallocator);
        frame_allocator = make_global(fallocator);
        init_alloc(page_allocator, frame_allocator);
    }

    println!("Hey");

    let mut vec = vec![5, 6, 9, 0];

    vec[0] = 3;

    println!("Works!");
    println!("{}", vec[0]);

    eop!()
}

/*
extern "x86-interrupt" fn breakpoint_handler(_stack_frame: &mut InterruptStackFrame)
{
    debug!("Breakpoint!");
    eop!()
}*/