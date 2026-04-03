# qemu rv64

## Qemu Arguments

``` shell
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
```

### -nographic

`-nographic` 表示模拟器不需要提供图形界面，而只需要对外输出字符流。仅限于串口模式输出的情况，如果启用了VGA输出，那么就什么也不会输出了。

### -bios

`-bios` 用于指定 Qemu 开机时的初始化引导程序。

## Qemu 启动

- 在Qemu模拟的 `virt` 硬件平台上，物理内存的起始物理地址为 `0x80000000` ，物理内存的默认大小为 128MiB.
- Qemu 上电从 `0x1000` 开始运行, 之后跳转到 `0x80000000`,执行bootloader.
- 如果使用上面给出的命令启动 Qemu ，那么在 Qemu **开始执行任何指令之前**，首先把两个文件加载到 Qemu 的物理内存中：即作把作为 bootloader 的 `rustsbi-qemu.bin` 加载到物理内存以物理地址 `0x80000000` 开头的区域上，同时把内核镜像 `os.bin` 加载到以物理地址 `0x80200000` 开头的区域上。
