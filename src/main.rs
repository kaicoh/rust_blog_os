#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use blog_os::println;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

entry_point!(kernal_main);

fn kernal_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::allocator;
    use blog_os::memory::{self, BootInfoFrameAllocator};

    println!("Hello World{}", "!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut v = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    println!("vec at {:p}", v.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_ref = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_ref)
    );
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_ref));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
