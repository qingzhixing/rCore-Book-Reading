#![no_std]
#![no_main]
mod lang_items;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    loop {}
}

fn clear_bss() {
    unsafe extern "C" {
        static sbss: u8;
        static ebss: u8;
    }

    let start_bss = ((unsafe { &sbss }) as *const u8) as usize;
    let end_bss = ((unsafe { &ebss }) as *const u8) as usize;

    for addr in start_bss..end_bss {
        unsafe {
            // 使用 write_volatile 防止编译器优化
            (addr as *mut u8).write_volatile(0);
        }
    }
}
