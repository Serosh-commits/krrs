use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB},
    VirtAddr,
};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: u64 = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 128 * 1024;

pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_alloc: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), &'static str> {
    let start = VirtAddr::new(HEAP_START);
    let end = start + HEAP_SIZE as u64 - 1u64;
    let start_page = Page::containing_address(start);
    let end_page = Page::containing_address(end);

    for page in Page::range_inclusive(start_page, end_page) {
        let frame = frame_alloc
            .allocate_frame()
            .ok_or("no more frames")?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_alloc).map_err(|_| "map failed")?.flush();
        }
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START as *mut u8, HEAP_SIZE);
    }

    Ok(())
}

pub fn used() -> usize {
    ALLOCATOR.lock().used()
}

pub fn free() -> usize {
    ALLOCATOR.lock().free()
}
