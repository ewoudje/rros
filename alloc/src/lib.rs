#![no_std]

extern crate ros_mem;
extern crate spin;

use core::alloc::{GlobalAlloc, Layout};
use ros_mem::alloc::PageAllocator;
use x86_64::structures::paging::{FrameAllocator, Size4KiB, PageSize};
use spin::Mutex;

pub struct MutexAlloc<'a>(Option<Mutex<Alloc<'a>>>);

impl<'a> MutexAlloc<'a> {
    pub unsafe fn new(offset: usize, palloc: &'a mut PageAllocator, falloc: &'a mut FrameAllocator<Size4KiB>) -> MutexAlloc<'a> {
        let mut entries: [*mut AllocEntry; 32] = [core::ptr::null_mut(); 32];

        for i in 0..32usize {
            let ptr = (i * 4096 + offset) as *mut AllocEntry;
            core::ptr::write(ptr, AllocEntry {
                next: 0,
                full: false,
                offset: i * 4096 + offset
            });
            entries[i] = ptr;
        };

        MutexAlloc(Some(Mutex::new(Alloc {
            page_alloc: palloc,
            frame_alloc: falloc,
            alloc_entries: entries
        })))
    }

    pub const unsafe fn empty() -> MutexAlloc<'static> {
        MutexAlloc(None)
    }
}

pub struct Alloc<'a> {
    page_alloc: &'a mut PageAllocator,
    frame_alloc: &'a mut FrameAllocator<Size4KiB>,
    alloc_entries: [*mut AllocEntry; 32]
}

pub struct AllocEntry {
    full: bool,
    offset: usize,
    next: u16
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct FullError;

impl AllocEntry {

    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, FullError>  {
        let alloc_start = align_up(self.next as usize, layout.align());
        let alloc_end = alloc_start.saturating_add(layout.size());

        if alloc_end <= 4096 {
            self.next = alloc_end as u16;
            Ok((alloc_start + self.offset) as *mut u8)
        } else {
            self.full = true;
            Err(FullError)
        }
    }
}

unsafe impl<'a> GlobalAlloc for MutexAlloc<'a> {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        let mut i = 0usize;
        loop {
            let entry = self.0.as_ref().unwrap().lock().alloc_entries[i];
            if !(*entry).full {
                break match (*entry).alloc(layout) {
                    Ok(t) => t,
                    Err(e) => self.alloc(layout)
                };
            }
            i += 1;
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //unimplemented!()
    }
}

/// Align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}