use core::mem::size_of;
use core::ptr::write;

static mut index: usize = 0;

pub unsafe fn make_global<T: Sized>(data: T) -> &'static mut T{
    let new = (0x0000_FFFF_0000 + index) as *mut T;
    write(new, data);
    index += size_of::<T>();
    return &mut *new;
}