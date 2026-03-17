fn main() {
    print_backtrace();
}

fn print_backtrace() {
    let bt = backtrace::Backtrace::new();
    println!("程序调用栈信息: \n{:?}", bt);
}
