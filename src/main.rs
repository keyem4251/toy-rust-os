#![no_std] // Rustの標準ライブラリをリンクしない
#![no_main] // Rustレベルのエントリポイントを無効化

use core::panic::PanicInfo;

mod vga_buffer;

// リンカが_startという関数を探すためエントリポイントを定義
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

// パニック時に呼ばれる関数を定義
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
