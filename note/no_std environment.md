# no_std 环境构建
## panic_handler

`#[panic_handler]` 用于定义 panic_handler 功能应对致命错误, 定义的panic_handler 具有 `fn(&PanicInfo) -> !` 函数签名

``` rust
#[panic_handler]

fn panic(info: &PanicInfo) -> ! {

    if let Some(location) = info.location() {

        println!(

            "Panic at {}: {} {}",

            location.file(),

            location.line(),

            info.message()

        );

    } else {

        println!("Panicked: {}", info.message());

    }

    shutdown(true);

}
```

## no_std
在 `riscv64gc-unknown-none-elf` 平台下编译程序需要使用 `#![no_std]` 移除对标准库的依赖。

## no_main
`#![no_main]` 让编译器不再需要完成对应平台上的初始化工作，有操作系统才能进行这些初始化，没有的话我们需要手动来实现。