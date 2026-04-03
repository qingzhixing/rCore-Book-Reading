# rust asm

## global_asm

`global_asm!()` 直接将汇编完整嵌入最终编译出来的 .out 文件中.

## include_str

将文件变成字符串导入 rust 文件中，类似 C 中的 `#include` .

``` rust
global_asm!(include_str!("entry.asm"));
```

我们通过 `include_str!` 宏将同目录下的汇编代码 `entry.asm` 转化为字符串并通过 `global_asm!` 宏嵌入到代码中。

## no_mangle
``` rust
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}
```
`#[no_mangle]` 将禁用 Rust 根据参数和返回值更改函数的签名，便于在汇编中调用 Rust 代码中的函数。