use x86_64::structures::paging::FrameAllocator;
use x86_64::structures::paging::Size4KiB;
use x86_64::structures::paging::PhysFrame;
use x86_64::structures::paging::Page;
use x86_64::VirtAddr;


pub fn init_frame_allocator<T: Iterator<Item = PhysFrame>>(frames: T) -> BumpFrameAllocator<impl Iterator<Item = PhysFrame>> {
    BumpFrameAllocator { frames }
}

pub fn init_page_allocator(start_address: u64) -> BumpPageAllocator {
    BumpPageAllocator {
        addr: VirtAddr::new(start_address)
    }
}

pub struct BumpPageAllocator {
    addr: VirtAddr
}

pub trait PageAllocator {
    fn allocate_page(&mut self) -> Page<Size4KiB>;
}

impl PageAllocator for BumpPageAllocator {
    fn allocate_page(&mut self) -> Page<Size4KiB> {
        let result = Page::from_start_address(self.addr);
        self.addr += 0x1000u64;
        result.unwrap()
    }
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub struct BumpFrameAllocator<I: Iterator<Item = PhysFrame>> {
    frames: I
}

unsafe impl<I: Iterator<Item = PhysFrame>> FrameAllocator<Size4KiB> for BumpFrameAllocator<I> {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.frames.next()
    }
}