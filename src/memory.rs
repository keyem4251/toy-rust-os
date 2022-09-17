use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PhysFrame, Size4KiB,
};
use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};

// ブートローダのメモリマップから使用可能なフレームを返すFrameAllocator
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// # Safety
    // 呼び出し元は渡されたメモリマップが有効であることを保証する必要がある
    // USABLEなフレームは実際に未使用でなければならない
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    // メモリマップによって指定されたusableなフレームのイテレータを返す
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // メモリマップからusableな領域を取得
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        // それぞれの領域をアドレス範囲に変換する
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());

        // フレームの開始アドレスのイテレータへと変換する
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/// # Safety
// 全物理メモリが渡された physical_memory_offset （だけずらしたうえ）で仮想メモリへとマップされていることを呼び出し元が保証しなければならない。
// また &mut 参照が複数の名称を持つこと（mutable aliasingといい、動作が未定義）につながるためこの関数は一度しか呼び出してはならない
// ページテーブルへの参照が可変（&mut）なので複数呼ばれると動作が不安定
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// # Safety
// 有効なレベル4テーブルへの参照を返す
// 全物理メモリが渡された physical_memory_offset （だけずらしたうえ）で仮想メモリへとマップされていることを呼び出し元が保証しなければならない。
// また &mut 参照が複数の名称を持つこと（mutable aliasingといい、動作が未定義）につながるためこの関数は一度しか呼び出してはならない
// ページテーブルへの参照が可変（&mut）なので複数呼ばれると動作が不安定
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    // Cr3レジスタから有効なレベル4テーブルの物理フレームを読む
    let (level_4_table_frame, _) = Cr3::read();

    // 開始アドレスに移動して物理アドレスを取り出す
    let phys = level_4_table_frame.start_address();

    // オフセットを足して仮想アドレスを取得
    // 呼び出し元がどのオフセットで仮想アドレスを取得できるかを保証
    let virt = physical_memory_offset + phys.as_u64();

    // 仮想アドレスの生ポインタを取得
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // 仮想アドレスのポインタ参照を可変で返す
}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe { mapper.map_to(page, frame, flags, frame_allocator) };
    map_to_result.expect("map_to failed").flush();
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}
