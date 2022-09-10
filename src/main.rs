#![no_std] // Rustの標準ライブラリをリンクしない
#![no_main] // Rustレベルのエントリポイントを無効化
#![feature(custom_test_frameworks)]
#![test_runner(toy_rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use toy_rust_os::println;

// リンカが_startという関数を探すためエントリポイントを定義
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    toy_rust_os::init();

    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

// パニック時に呼ばれる
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    toy_rust_os::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
