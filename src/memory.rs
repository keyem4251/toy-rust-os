use x86_64::{structures::paging::PageTable, VirtAddr};

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
