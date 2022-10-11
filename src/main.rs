#![no_std] // Rustの標準ライブラリをリンクしない
#![no_main] // Rustレベルのエントリポイントを無効化
#![feature(custom_test_frameworks)]
#![test_runner(toy_rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use toy_rust_os::task::{executor::Executor, keyboard, Task};
use toy_rust_os::{allocator, println};

// bootloaderクレートによりkernel_mainの引数の型を確認しエントリポイントとして定義
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use toy_rust_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World!");
    toy_rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keyboard()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    toy_rust_os::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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
