#![no_std] // Rustの標準ライブラリをリンクしない
#![no_main] // Rustレベルのエントリポイントを無効化

use core::panic::PanicInfo;

mod vga_buffer;

// リンカが_startという関数を探すためエントリポイントを定義
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}

// パニック時に呼ばれる関数を定義
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
