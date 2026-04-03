# Macros

`#[macro_export]` 用于导出在模块中定义的宏。

``` rust
// console.rs
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}
```

`#[macro_use]` 用于导入模块中定义的宏。

``` rust
// main.rs
#[macro_use]
mod console;
```
