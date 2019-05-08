use ros_mem::alloc::PageAllocator;
use x86_64::structures::paging::{FrameAllocator, Size4KiB};
use ros_alloc::MutexAlloc;

pub unsafe fn init_alloc<PA: PageAllocator + 'static, FA: FrameAllocator<Size4KiB> + 'static>(palloc: &'static mut PA, falloc: &'static mut FA) {
    crate::ALLOC = MutexAlloc::new(0x0001_0000_0000, palloc, falloc)
}