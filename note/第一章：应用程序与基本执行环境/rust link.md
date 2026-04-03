# Rust Link

``` text
 // os/.cargo/config
 
 [target.riscv64gc-unknown-none-elf] rustflags = [     "-Clink-arg=-Tsrc/linker.ld", "-Cforce-frame-pointers=yes" ]
```

修改 Cargo 的配置文件来使用我们自己的链接脚本 `os/src/linker.ld` 而非使用默认的内存布局
