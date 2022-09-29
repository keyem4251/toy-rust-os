use super::align_up;
use core::mem;

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        LinkedListAllocator { head: ListNode::new(0) }
    }

    /// # Safety
    // 与えられたヒープの位置、サイズで初期化する
    // 呼び出し側で有効なメモリの範囲を指定する必要があるためunsafe
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    // 空いているメモリ領域をLinkedListに追加
    // initでしか呼んでいないがメモリを開放するdeallocでも呼ばれる可能性がある
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // 空いているメモリ領域がListNodeで確保できることの確認
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        // 新しいListNodeを作成しlistに追加する
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr)
    }

    // 空き領域を探して、LinkedListから外してListNodeを返す
    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        // 現在のlist nodeを取得する
        let mut current = &mut self.head;

        // list nodeをループしlinked listの中でメモリが十分空いているものを探す
        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(region, size, align) {
                // 空き領域が見つかった場合linked listから削除する
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));
                current.next = next;
                return ret;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        // size, alignに合う空き領域が見つからない
        None
    }

    // 渡されたLinkedNode（空き領域）に割当を行う
    // 割当可能ならメモリの位置を返す
    fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            // 領域が小さい
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            // ListNodeの保持している領域の残りが小さい
            return Err(());
        }

        // 割当のための領域確保
        Ok(alloc_start)
    }
}

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}
