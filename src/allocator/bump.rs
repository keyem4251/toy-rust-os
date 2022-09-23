pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
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
