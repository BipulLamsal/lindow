use lazy_static::lazy_static;
use x86_64::{
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

pub const DOUBLE_FAULT_INDEX: u16 = 1; // Backup stack index when kernel stack overflows

lazy_static! {
    static ref TSS: TaskStateSegment = {
            let mut tss = TaskStateSegment::new();
            tss.interrupt_stack_table[DOUBLE_FAULT_INDEX as usize] = {
                // the stack grows downward i.e high address (stack_end) → low address (stack_start)
                const STACK_SIZE: usize = 4096 * 5; // 1 page =  4KB
                static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
                let stack_start = VirtAddr::from_ptr(&raw const STACK);
                let stack_end = stack_start + STACK_SIZE;
                stack_end // this address is the top of the index , there is no guard as well
            };
            tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selector) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector  = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

        (gdt, Selector{cs: code_selector, tss: tss_selector})
        // ldgt (local register table register) loads the 'static ref or memory address of the gdt
        // descriptor
    };
}
struct Selector {
    cs: SegmentSelector,
    tss: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.cs);
        load_tss(GDT.1.tss);
    }
}
