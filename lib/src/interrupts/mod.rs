use x86_64::structures::idt::{ InterruptDescriptorTable, ExceptionStackFrame };
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{ GlobalDescriptorTable, Descriptor, SegmentSelector };
use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::VirtAddr;
use lazy_static::lazy_static;
use spin::Mutex;

pub mod pic;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };

    static ref IDT_prepare: Mutex<fn(&mut InterruptDescriptorTable)> = Mutex::new(|_func: &mut InterruptDescriptorTable| {});

    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        pic::prepare(&mut idt);
        IDT_prepare.lock()(&mut idt);
        idt
    };

    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init_interrupts(func: fn(&mut InterruptDescriptorTable)) {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;
    GDT.0.load();
    unsafe {
        set_cs(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
    *IDT_prepare.lock() = func;
    IDT.load();
    unsafe { pic::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame)
{
    debug!("EXCEPTION: BREAKPOINT (DEFAULT (not assigned))\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame, _error_code: u64)
{
    debug!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    eop!()
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut ExceptionStackFrame, error_code: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;

    debug!("ERROR CODE:\n {:?}", error_code);
    debug!("ACCESSED ADDRESS: {:?}", Cr2::read());
    debug!("EXCEPTION: PAGE FAULT\n{:#?}", stack_frame);
    eop!()
}