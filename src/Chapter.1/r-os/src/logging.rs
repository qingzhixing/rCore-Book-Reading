use log::{self, Level, LevelFilter, Log, Metadata, Record};

struct Logger;

impl Log for Logger {
    // 所有日志都可以通过，后续根据日志等级过滤
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };

        println!(
            "\u{001b}[{}m[{:>5}] {}\u{001b}[0m",
            color,
            record.level(),
            record.args()
        );
    }

    fn flush(&self) {
        // No buffering, so nothing to flush
    }
}

static LOGGER: Logger = Logger;

pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    // 编译期获取参数 LOG_LEVEL，默认值为 info
    log::set_max_level(match option_env!("LOG_LEVEL") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        Some("off") => LevelFilter::Off,
        _ => LevelFilter::Info,
    });
}
