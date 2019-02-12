use bootloader_precompiled::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{Mapper, Page, PageTable, RecursivePageTable};
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};

pub fn init(level_4_table_addr: usize) -> RecursivePageTable<'static> {
    let level_4_table_ptr = level_4_table_addr as *mut PageTable;
    unsafe {
        let level_4_table = &mut *level_4_table_ptr;
        RecursivePageTable::new(level_4_table).unwrap()
    }
}

pub struct BootInfoFrameAllocator<I> where I: Iterator<Item=PhysFrame> {
    frames: I,
}

impl<I> FrameAllocator<Size4KiB> for BootInfoFrameAllocator<I>
    where I: Iterator<Item=PhysFrame>
{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.frames.next()
    }
}

pub fn init_frame_allocator(
    memory_map: &'static MemoryMap,
) -> BootInfoFrameAllocator<impl Iterator<Item=PhysFrame>> {
    let regions = memory_map
        .iter()
        .filter(|r| r.region_type == MemoryRegionType::Usable);
    let addr_ranges = regions.map(|r| r.range.start_addr()..r.range.end_addr());
    let frame_addresses = addr_ranges.flat_map(|r| r.into_iter().step_by(4096));
    let frames = frame_addresses.map(|addr| {
        PhysFrame::containing_address(PhysAddr::new(addr))
    });

    BootInfoFrameAllocator { frames }
}

pub fn create_mapping(
    recursive_page_table: &mut RecursivePageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let page: Page = Page::containing_address(VirtAddr::new(0xdeadbeaf900));
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        recursive_page_table.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}