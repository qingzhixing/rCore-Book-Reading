pub fn console_putchar(ch: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(ch);
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{NoReason, Shutdown, SystemFailure, system_reset};
    if failure {
        system_reset(Shutdown, SystemFailure);
    } else {
        system_reset(Shutdown, NoReason);
    }
    unreachable!()
}
