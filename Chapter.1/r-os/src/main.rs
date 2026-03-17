#![deny(warnings)]
#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

use core::arch::global_asm;
use log::*;

use crate::sbi::shutdown;
global_asm!(include_str!("entry.asm"));

// 在 entry.asm 中被调用，使用需要 no_mangle 防止符号名被修改
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello Rust OS!");
    logging::init_logger();
    error!("This is an error");
    warn!("This is a warning");
    info!("This is an info");
    debug!("This is a debug");
    trace!("This is a trace");
    shutdown(false);
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
