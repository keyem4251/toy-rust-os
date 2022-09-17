use x86_64::{structures::paging::PageTable, VirtAddr, PhysAddr};

/// # Safety
// 有効なレベル4テーブルへの参照を返す
// 全物理メモリが渡された physical_memory_offset （だけずらしたうえ）で仮想メモリへとマップされていることを呼び出し元が保証しなければならない。
// また &mut 参照が複数の名称を持つこと（mutable aliasingといい、動作が未定義）につながるためこの関数は一度しか呼び出してはならない
// ページテーブルへの参照が可変（&mut）なので複数呼ばれると動作が不安定
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
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

/// # Safety
// 与えられた仮想アドレスを対応する物理アドレスに変換し、アドレスがマップされていない場合はNoneを返す
// 全物理メモリが渡された physical_memory_offset （だけずらしたうえ）で仮想メモリへとマップされていることを呼び出し元が保証しなければならない。
pub unsafe fn transalate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    // Cr3レジスタから有効なレベル4テーブルの物理フレームを読む
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()];
    let mut frame = level_4_table_frame;

    // 複数層のページテーブルをたどる
    for &index in &table_indexes {
        // フレームをページテーブルの参照に変換する
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {&*table_ptr};

        // ページテーブルエントリを読んでframeを更新する
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    // ページオフセットを足して目的の物理アドレスを計算する
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
