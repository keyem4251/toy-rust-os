#![no_std] // Rustの標準ライブラリをリンクしない
#![no_main] // Rustレベルのエントリポイントを無効化
#![feature(custom_test_frameworks)]
#![test_runner(toy_rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use toy_rust_os::{memory::transalate_addr, println};

// bootloaderクレートによりkernel_mainの引数の型を確認しエントリポイントとして定義
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use toy_rust_os::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

    println!("Hello World!");
    toy_rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    toy_rust_os::hlt_loop();
}

// パニック時に呼ばれる
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    toy_rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    toy_rust_os::test_panic_handler(info);
}

// #[test_case]
// fn trivial_assertion() {
//     assert_eq!(1, 1);
// }
