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

    println!("[kernel] Hello Rust OS!");

    logging::init_logger();
    logger_test_print();

    print_kernel_section();

    shutdown(false);
}

fn logger_test_print() {
    error!("This is an error");
    warn!("This is a warning");
    info!("This is an info");
    debug!("This is a debug");
    trace!("This is a trace");
}

#[allow(function_casts_as_integer)]
fn print_kernel_section() {
    unsafe extern "C" {
        safe fn skernel(); // Start of kernel
        safe fn stext(); // Start of text section
        safe fn etext(); // End of text section
        safe fn srodata(); // Start of read-only data section
        safe fn erodata(); // End of read-only data section
        safe fn sdata(); // Start of data section
        safe fn edata(); // End of data section
        safe fn sbss(); // Start of bss section
        safe fn ebss(); // End of bss section
        safe fn ekernel(); // End of kernel
        safe fn boot_stack_lower_bound(); // Lower bound of boot stack
        safe fn boot_stack_top(); // Upper bound of boot stack
    }
    info!(
        "[kernel] .text section: [{:#x}, {:#x})",
        stext as usize, etext as usize
    );
    info!(
        "[kernel] .rodata section: [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data section: [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    info!(
        "[kernel] .bss section: [{:#x}, {:#x})",
        sbss as usize, ebss as usize
    );
    info!(
        "[kernel] .kernel section: [{:#x}, {:#x})",
        skernel as usize, ekernel as usize
    );
    info!(
        "[kernel] stack range: top==bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
}

#[allow(function_casts_as_integer)]
fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    for addr in (sbss as usize)..(ebss as usize) {
        unsafe {
            // 使用 write_volatile 防止编译器优化
            (addr as *mut u8).write_volatile(0);
        }
    }
}
