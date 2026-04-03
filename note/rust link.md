# Rust Link

## Linker Script

``` text
 // os/.cargo/config
 
 [target.riscv64gc-unknown-none-elf] rustflags = [     "-Clink-arg=-Tsrc/linker.ld", "-Cforce-frame-pointers=yes" ]
```

修改 Cargo 的配置文件来使用我们自己的链接脚本 `os/src/linker.ld` 而非使用默认的内存布局

## Link Section

``` rust
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}
```

`#[link_section = ".text.entry"]` 可以指定该函数编译后的汇编代码放在 `.text.entry` 代码段中.

## Weak Link
``` rust
// lib.rs
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
```
若用户代码中定义了 main() 则会覆盖它，否则使用这个 main. 用于检测用户程序是否有 main 函数。仅会在运行时报错。
要使用这条特性需要启用 `#![feature(linkage)]`
