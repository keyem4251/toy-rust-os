use alloc::alloc::{GlobalAlloc, Layout}

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize, // 最後の領域が開放されたあとにアロケーターをリセットするためのカウンター
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator { heap_start: 0, heap_end: 0, next: 0, allocations: 0 }
    }

    // 与えられたヒープの位置、サイズで初期化する
    // 呼び出し側で有効なメモリの範囲を指定する必要があるためunsafe
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO alignment and bounds check
        let alloc_start = self.next;
        self.next = alloc_start + layout.size();
        self.allocations += 1;
        alloc_start as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!();
    }
}
